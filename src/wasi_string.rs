use std::borrow::Cow;
use std::ffi::CStr;
use std::str;

/// A utility for converting from outside-world POSIX-oriented strings, such
/// as command-line arguments and environment variables, into UTF-8 strings,
/// using the ARF encoding technique to handle unencodable byte sequences.
pub struct WasiString<'str>(Cow<'str, str>);

impl<'str> WasiString<'str> {
    /// Construct a `WasiString` with data copied from the given `&CStr`,
    /// using ARF encoding as needed to ensure that the result is valid UTF-8.
    pub fn from_maybe_nonutf8_cstr(cstr: &'str CStr) -> Self {
        let bytes = cstr.to_bytes();
        if let Ok(s) = str::from_utf8(bytes) {
            return Self(Cow::Borrowed(s));
        }

        Self::from_nonutf8_cstr(bytes)
    }

    /// Slow path for `from_maybe_nonutf8_cstr`.
    fn from_nonutf8_cstr(bytes: &[u8]) -> Self {
        let mut data = String::new();

        data.push('\u{feff}');

        let mut input = bytes;
        loop {
            match std::str::from_utf8(input) {
                Ok(valid) => {
                    data.push_str(valid);
                    break;
                }
                Err(error) => {
                    let (valid, after_valid) = input.split_at(error.valid_up_to());
                    unsafe { data.push_str(str::from_utf8_unchecked(valid)) }
                    data.push('\u{FFFD}');

                    if let Some(invalid_sequence_length) = error.error_len() {
                        input = &after_valid[invalid_sequence_length..]
                    } else {
                        break;
                    }
                }
            }
        }

        data.push('\0');

        let mut input = bytes;
        loop {
            match std::str::from_utf8(input) {
                Ok(valid) => {
                    data.push_str(valid);
                    break;
                }
                Err(error) => {
                    let (valid, after_valid) = input.split_at(error.valid_up_to());
                    unsafe { data.push_str(str::from_utf8_unchecked(valid)) }
                    data.push('\0');
                    data.push((after_valid[0] & 0x7f) as char);

                    if error.error_len().is_some() {
                        input = &after_valid[1..]
                    } else {
                        break;
                    }
                }
            }
        }

        Self(Cow::Owned(data))
    }

    /// Return a reference to the valid UTF-8 contents.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[test]
fn valid_utf8() {
    assert_eq!(
        WasiString::from_maybe_nonutf8_cstr(CStr::from_bytes_with_nul(b"\0").unwrap()).as_str(),
        ""
    );
    assert_eq!(
        WasiString::from_maybe_nonutf8_cstr(CStr::from_bytes_with_nul(b"foo\0").unwrap()).as_str(),
        "foo"
    );
}

#[test]
fn not_utf8() {
    assert_eq!(
        WasiString::from_maybe_nonutf8_cstr(CStr::from_bytes_with_nul(b"\xfe\0").unwrap()).as_str(),
        "\u{feff}\u{fffd}\0\0\u{7e}"
    );
    assert_eq!(
        WasiString::from_maybe_nonutf8_cstr(CStr::from_bytes_with_nul(b"\xc0\xff\0").unwrap())
            .as_str(),
        "\u{feff}\u{fffd}\u{fffd}\0\0\u{40}\0\u{7f}"
    );
    assert_eq!(
        WasiString::from_maybe_nonutf8_cstr(CStr::from_bytes_with_nul(b"\xef\xbb\xbf\0").unwrap())
            .as_str(),
        "\u{feff}"
    );
    assert_eq!(
        WasiString::from_maybe_nonutf8_cstr(
            CStr::from_bytes_with_nul(b"\xef\xbb\xbf\xfd\0").unwrap()
        )
        .as_str(),
        "\u{feff}\u{feff}\u{fffd}\0\u{feff}\0\x7d"
    );
}
