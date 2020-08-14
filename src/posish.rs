#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr},
    io, str,
};

/// Convert a byte sequence which is either plain UTF-8 or an ARF encoding into
/// a `CString` ready for use in POSIX-style APIs.
pub fn bytes_to_host(bytes: &[u8]) -> io::Result<CString> {
    let s = str::from_utf8(bytes).map_err(|_| encoding_error())?;
    str_to_host(s)
}

/// Convert a `&str` which is either plain UTF-8 or an ARF encoding into a
/// `CString` ready for use in POSIX-style APIs.
pub fn str_to_host(s: &str) -> io::Result<CString> {
    match CString::new(s) {
        Ok(c_string) => Ok(c_string),
        Err(e) => from_arf(s, e.nul_position()),
    }
}

/// Convert an `&OsStr` produced by POSIX-style APIs into a `Cow<str>` which
/// is either plain UTF-8 or an ARF encoding. Returns an error if the input
/// string contains NUL bytes.
pub fn host_os_str_to_str(host: &OsStr) -> io::Result<Cow<str>> {
    if host.as_bytes().contains(&b'\0') {
        return Err(encoding_error());
    }
    Ok(if let Ok(s) = str::from_utf8(host.as_bytes()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(to_arf(host.as_bytes()))
    })
}

/// Convert an `&OsStr` produced by POSIX-style APIs into a `Cow<[u8]>` which
/// is either plain UTF-8 or an ARF encoding. Returns an error if the input
/// string contains NUL bytes.
pub fn host_os_str_to_bytes(host: &OsStr) -> io::Result<Cow<[u8]>> {
    Ok(match host_os_str_to_str(host)? {
        Cow::Borrowed(b) => Cow::Borrowed(b.as_bytes()),
        Cow::Owned(b) => Cow::Owned(b.into_bytes()),
    })
}

/// Convert an `&CStr` produced by POSIX-style APIs into a `Cow<str>` which
/// is either plain UTF-8 or an ARF encoding.
pub fn host_c_str_to_str(host: &CStr) -> Cow<str> {
    if let Ok(s) = str::from_utf8(host.to_bytes()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(to_arf(host.to_bytes()))
    }
}

/// Convert an `&CStr` produced by POSIX-style APIs into a `Cow<[u8]>` which
/// is either plain UTF-8 or an ARF encoding.
pub fn host_c_str_to_bytes(host: &CStr) -> Cow<[u8]> {
    let bytes = host_c_str_to_str(host);
    match bytes {
        Cow::Borrowed(b) => Cow::Borrowed(b.as_bytes()),
        Cow::Owned(b) => Cow::Owned(b.into_bytes()),
    }
}

/// Slow path for `str_to_host`.
#[cold]
fn from_arf(s: &str, nul: usize) -> io::Result<CString> {
    if !s.starts_with('\u{feff}') {
        return Err(encoding_error());
    }

    let mut lossy = s.bytes().skip('\u{feff}'.len_utf8());
    let mut nul_escaped = s.bytes().skip(nul + 1);
    let mut any_invalid = false;
    let mut vec = Vec::new();
    while let Some(b) = nul_escaped.next() {
        if b == b'\0' {
            let more = nul_escaped.next().ok_or_else(encoding_error)?;
            if (more & 0x80) != 0 {
                return Err(encoding_error());
            }
            // Test for U+FFFD.
            let l0 = lossy.next().ok_or_else(encoding_error)?;
            let l1 = lossy.next().ok_or_else(encoding_error)?;
            let l2 = lossy.next().ok_or_else(encoding_error)?;
            if [l0, l1, l2] != [0xef, 0xbf, 0xbd] {
                return Err(encoding_error());
            }
            any_invalid = true;
            vec.push(more | 0x80);
        } else {
            if lossy.next() != Some(b) {
                return Err(encoding_error());
            }
            vec.push(b);
        }
    }
    if !any_invalid {
        return Err(encoding_error());
    }
    if lossy.next() != Some(b'\0') {
        return Err(encoding_error());
    }

    // Validation succeeded.
    Ok(unsafe { CString::from_vec_unchecked(vec) })
}

/// Slow path for `host_to_bytes`.
#[cold]
fn to_arf(bytes: &[u8]) -> String {
    let mut data = String::new();

    data.push('\u{feff}');

    let mut input = bytes;

    // This loop and `unsafe` follow the example in the documentation:
    // https://doc.rust-lang.org/stable/std/str/struct.Utf8Error.html#examples
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

                if let Some((_, remaining)) = after_valid.split_first() {
                    input = remaining;
                } else {
                    break;
                }
            }
        }
    }

    data.push('\0');

    // This loop and `unsafe` follow the example in the documentation
    // mentioned above.
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
                if let Some((byte, remaining)) = after_valid.split_first() {
                    data.push('\0');
                    data.push((byte & 0x7f) as char);
                    input = remaining;
                } else {
                    break;
                }
            }
        }
    }

    data
}

#[cold]
fn encoding_error() -> io::Error {
    io::Error::from_raw_os_error(libc::EILSEQ)
}

#[test]
fn utf8_inputs() {
    assert_eq!(str_to_host("").unwrap().to_bytes(), b"");
    assert_eq!(str_to_host("f").unwrap().to_bytes(), b"f");
    assert_eq!(str_to_host("foo").unwrap().to_bytes(), b"foo");
    assert_eq!(
        str_to_host("\u{fffd}").unwrap().to_bytes(),
        "\u{fffd}".as_bytes()
    );
    assert_eq!(
        str_to_host("\u{fffd}foo").unwrap().to_bytes(),
        "\u{fffd}foo".as_bytes()
    );
    assert_eq!(
        str_to_host("\u{feff}foo").unwrap().to_bytes(),
        "\u{feff}foo".as_bytes()
    );
}

#[test]
fn arf_inputs() {
    assert_eq!(
        str_to_host("\u{feff}hello\u{fffd}world\0hello\0\x05world")
            .unwrap()
            .to_bytes(),
        b"hello\x85world"
    );
    assert_eq!(
        str_to_host("\u{feff}hello\u{fffd}\0hello\0\x05")
            .unwrap()
            .to_bytes(),
        b"hello\x85"
    );
}

#[test]
fn errors_from_bytes() {
    assert!(bytes_to_host(b"\xfe").is_err());
    assert!(bytes_to_host(b"\xc0\xff").is_err());
}

#[test]
fn errors_from_str() {
    assert!(str_to_host("\u{feff}hello world\0hello world").is_err());
    assert!(str_to_host("\u{feff}hello world\0\0hello world\0").is_err());
    assert!(str_to_host("\u{feff}hello\u{fffd}world\0\0hello\0\x05world\0").is_err());
    assert!(str_to_host("\u{fffe}hello\u{fffd}world\0hello\0\x05world").is_err());
    assert!(str_to_host("\u{feff}hello\u{fffd}\0hello\0").is_err());
}

#[test]
fn valid_utf8() {
    assert_eq!(host_os_str_to_str(OsStr::from_bytes(b"")).unwrap(), "");
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(b"foo")).unwrap(),
        "foo"
    );

    // Same thing, now with `CStr`s.
    assert_eq!(
        host_c_str_to_str(CStr::from_bytes_with_nul(b"\0").unwrap()),
        ""
    );
    assert_eq!(
        host_c_str_to_str(CStr::from_bytes_with_nul(b"foo\0").unwrap()),
        "foo"
    );
}

#[test]
fn not_utf8() {
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(b"\xfe")).unwrap(),
        "\u{feff}\u{fffd}\0\0\u{7e}"
    );
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(b"\xc0\xff")).unwrap(),
        "\u{feff}\u{fffd}\u{fffd}\0\0\u{40}\0\u{7f}"
    );
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(b"\xef\xbb\xbf")).unwrap(),
        "\u{feff}"
    );
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(b"\xef\xbb\xbf\xfd")).unwrap(),
        "\u{feff}\u{feff}\u{fffd}\0\u{feff}\0\x7d"
    );

    // Same thing, now with `CStr`s.
    assert_eq!(
        host_c_str_to_str(CStr::from_bytes_with_nul(b"\xfe\0").unwrap()),
        "\u{feff}\u{fffd}\0\0\u{7e}"
    );
    assert_eq!(
        host_c_str_to_str(CStr::from_bytes_with_nul(b"\xc0\xff\0").unwrap()),
        "\u{feff}\u{fffd}\u{fffd}\0\0\u{40}\0\u{7f}"
    );
    assert_eq!(
        host_c_str_to_str(CStr::from_bytes_with_nul(b"\xef\xbb\xbf\0").unwrap()),
        "\u{feff}"
    );
    assert_eq!(
        host_c_str_to_str(CStr::from_bytes_with_nul(b"\xef\xbb\xbf\xfd\0").unwrap()),
        "\u{feff}\u{feff}\u{fffd}\0\u{feff}\0\x7d"
    );
}

#[test]
fn round_trip() {
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(bytes_to_host(b"").unwrap().as_bytes())).unwrap(),
        ""
    );
    assert_eq!(
        host_os_str_to_str(OsStr::from_bytes(
            bytes_to_host(b"hello").unwrap().as_bytes()
        ))
        .unwrap(),
        "hello"
    );
    assert_eq!(
        str_to_host(&host_os_str_to_str(OsStr::from_bytes(b"hello")).unwrap())
            .unwrap()
            .as_bytes(),
        b"hello"
    );
    assert_eq!(
        str_to_host(&host_os_str_to_str(OsStr::from_bytes(b"h\xc0ello\xc1")).unwrap())
            .unwrap()
            .as_bytes(),
        b"h\xc0ello\xc1"
    );
    assert_eq!(
        str_to_host(&host_os_str_to_str(OsStr::from_bytes(b"\xf5\xff")).unwrap())
            .unwrap()
            .as_bytes(),
        b"\xf5\xff"
    );
    assert_eq!(
        str_to_host(&host_os_str_to_str(OsStr::from_bytes(b"")).unwrap())
            .unwrap()
            .as_bytes(),
        b""
    );
    assert_eq!(
        str_to_host(&host_os_str_to_str(OsStr::from_bytes(b"\xe6\x96")).unwrap())
            .unwrap()
            .as_bytes(),
        b"\xe6\x96"
    );

    // Same thing, now with `CStr`s.
    assert_eq!(
        str_to_host(&host_c_str_to_str(
            CStr::from_bytes_with_nul(b"hello\0").unwrap()
        ))
        .unwrap()
        .as_bytes(),
        b"hello"
    );
    assert_eq!(
        str_to_host(&host_c_str_to_str(
            CStr::from_bytes_with_nul(b"h\xc0ello\xc1\0").unwrap()
        ))
        .unwrap()
        .as_bytes(),
        b"h\xc0ello\xc1"
    );
    assert_eq!(
        str_to_host(&host_c_str_to_str(
            CStr::from_bytes_with_nul(b"\xf5\xff\0").unwrap()
        ))
        .unwrap()
        .as_bytes(),
        b"\xf5\xff"
    );
    assert_eq!(
        str_to_host(&host_c_str_to_str(
            CStr::from_bytes_with_nul(b"\0").unwrap()
        ))
        .unwrap()
        .as_bytes(),
        b""
    );
    assert_eq!(
        str_to_host(&host_c_str_to_str(
            CStr::from_bytes_with_nul(b"\xe6\x96\0").unwrap()
        ))
        .unwrap()
        .as_bytes(),
        b"\xe6\x96"
    );
}
