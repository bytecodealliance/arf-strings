use std::{
    ffi::{CStr, CString},
    str,
};

/// A NUL-terminated and not-necessarily-UTF-8 string.
///
/// A utility for converting from inside-world path-oriented strings, such
/// as file and directory names, into NUL-terminated byte strings that can
/// be passed to POSIX-like APIs. Input strings containing NUL bytes are
/// interpreted as ARF strings.
pub struct PosixString(CString);

impl PosixString {
    /// Construct a `PosixString` from data in the given byte slice, which should
    /// contain a valid UTF-8 string, which must either contain no NUL bytes,
    /// or must be a valid ARF string.
    ///
    /// If the data is not valid UTF-8, or if it contains NUL bytes and is not
    /// valid ARF, return an error.
    pub fn from_path_bytes(bytes: &[u8]) -> Result<Self, ()> {
        let s = str::from_utf8(bytes).map_err(|_| ())?;
        Self::from_path_str(s)
    }

    /// Construct a `PosixString` from data in the given `&str`, which must either
    /// contain no NUL bytes, or must be a valid ARF string.
    ///
    /// If the data contains NUL bytes and is not valid ARF, return an error.
    pub fn from_path_str(s: &str) -> Result<Self, ()> {
        match CString::new(s) {
            Ok(cstring) => Ok(Self(cstring)),
            Err(e) => Self::from_arf(s, e.nul_position()),
        }
    }

    /// Slow path for `from_path_str`.
    #[cold]
    fn from_arf(s: &str, nul: usize) -> Result<Self, ()> {
        if !s.starts_with('\u{feff}') {
            return Err(());
        }

        let mut lossy = s.bytes().skip('\u{feff}'.len_utf8());
        let mut nul_escaped = s.bytes().skip(nul + 1);
        let mut any_invalid_bytes = false;
        let mut vec = Vec::new();
        while let Some(b) = nul_escaped.next() {
            if b == b'\0' {
                let more = nul_escaped.next().ok_or(())?;
                if (more & 0x80) != 0 {
                    return Err(());
                }
                // Test for U+FFFD.
                let l0 = lossy.next().ok_or(())?;
                let l1 = lossy.next().ok_or(())?;
                let l2 = lossy.next().ok_or(())?;
                if [l0, l1, l2] != [0xef, 0xbf, 0xbd] {
                    return Err(());
                }
                any_invalid_bytes = true;
                vec.push(more | 0x80);
            } else {
                if lossy.next() != Some(b) {
                    return Err(());
                }
                vec.push(b);
            }
        }
        if !any_invalid_bytes {
            return Err(());
        }
        if lossy.next() != Some(b'\0') {
            return Err(());
        }

        // Validation succeeded.
        Ok(Self(unsafe { CString::from_vec_unchecked(vec) }))
    }

    /// Return a `&CStr` reference to the contained `CString`.
    pub fn as_cstr(&self) -> &CStr {
        &self.0
    }

    /// Consume this `PosixString` and return the contained `CString`.
    pub fn into_cstring(self) -> CString {
        self.0
    }
}

#[test]
fn utf8_inputs() {
    assert_eq!(
        PosixString::from_path_str("").unwrap().as_cstr().to_bytes(),
        b""
    );
    assert_eq!(
        PosixString::from_path_str("f")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        b"f"
    );
    assert_eq!(
        PosixString::from_path_str("foo")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        b"foo"
    );
    assert_eq!(
        PosixString::from_path_str("\u{fffd}")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        "\u{fffd}".as_bytes()
    );
    assert_eq!(
        PosixString::from_path_str("\u{fffd}foo")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        "\u{fffd}foo".as_bytes()
    );
    assert_eq!(
        PosixString::from_path_str("\u{feff}foo")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        "\u{feff}foo".as_bytes()
    );
}

#[test]
fn arf_inputs() {
    assert_eq!(
        PosixString::from_path_str("\u{feff}hello\u{fffd}world\0hello\0\x05world")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        b"hello\x85world"
    );
    assert_eq!(
        PosixString::from_path_str("\u{feff}hello\u{fffd}\0hello\0\x05")
            .unwrap()
            .as_cstr()
            .to_bytes(),
        b"hello\x85"
    );
}

#[test]
fn errors_from_bytes() {
    assert!(PosixString::from_path_bytes(b"\xfe").is_err());
    assert!(PosixString::from_path_bytes(b"\xc0\xff").is_err());
}

#[test]
fn errors_from_str() {
    assert!(PosixString::from_path_str("\u{feff}hello world\0hello world").is_err());
    assert!(PosixString::from_path_str("\u{feff}hello world\0\0hello world\0").is_err());
    assert!(
        PosixString::from_path_str("\u{feff}hello\u{fffd}world\0\0hello\0\x05world\0").is_err()
    );
    assert!(PosixString::from_path_str("\u{fffe}hello\u{fffd}world\0hello\0\x05world").is_err());
    assert!(PosixString::from_path_str("\u{feff}hello\u{fffd}\0hello\0").is_err());
}
