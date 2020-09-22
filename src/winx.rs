use std::{
    char::decode_utf16,
    convert::TryFrom,
    ffi::{OsStr, OsString},
    io,
    os::windows::ffi::{OsStrExt, OsStringExt},
    str,
};

/// Convert a byte sequence which is either plain UTF-8 or an ARF encoding into
/// a `OsString` ready for use in Windows-style APIs.
pub fn bytes_to_host(bytes: &[u8]) -> io::Result<OsString> {
    let s = str::from_utf8(bytes).map_err(|_| encoding_error())?;
    str_to_host(s)
}

/// Convert a `&str` which is either plain UTF-8 or an ARF encoding into a
/// `OsString` ready for use in Windows-style APIs.
pub fn str_to_host(s: &str) -> io::Result<OsString> {
    if let Some(nul_position) = s.chars().position(|c| c == '\0') {
        from_arf(s, nul_position)
    } else {
        Ok(OsString::from_wide(&s.encode_utf16().collect::<Vec<_>>()))
    }
}

/// Convert an `&OsStr` produced by Windows-style APIs into a `Cow<str>` which
/// is either plain UTF-8 or an ARF encoding.
pub fn host_to_str(host: &OsStr) -> io::Result<String> {
    let wide = host.encode_wide().collect::<Vec<_>>();
    if wide.contains(&0) {
        return Err(encoding_error());
    }
    Ok(if let Ok(s) = String::from_utf16(&wide) {
        s
    } else {
        to_arf(&wide)
    })
}

/// Convert an `&OsStr` produced by Windows-style APIs into a `Cow<[u8]>` which
/// is either plain UTF-8 or an ARF encoding.
pub fn host_to_bytes(host: &OsStr) -> io::Result<Vec<u8>> {
    host_to_str(host).map(String::into_bytes)
}

/// Slow path for `str_to_host`.
#[cold]
fn from_arf(s: &str, nul: usize) -> io::Result<OsString> {
    let mut lossy = s.chars();
    if lossy.next() != Some('\u{feff}') {
        return Err(encoding_error());
    }

    let mut nul_escaped = s.chars().skip(nul + 1);
    let mut any_invalid = false;
    let mut vec = Vec::new();
    while let Some(c) = nul_escaped.next() {
        if c == '\0' {
            let more = nul_escaped.next().ok_or_else(encoding_error)?;
            if more > '\u{7ff}' {
                return Err(encoding_error());
            }
            // Test for U+FFFD.
            let l = lossy.next().ok_or_else(encoding_error)?;
            if l != '\u{fffd}' {
                return Err(encoding_error());
            }
            any_invalid = true;
            let unit = u16::try_from((more as u16) + 0xd800).map_err(|_| encoding_error())?;
            vec.push(unit);
        } else {
            if lossy.next() != Some(c) {
                return Err(encoding_error());
            }
            let mut buf = [0; 2];
            let utf16 = c.encode_utf16(&mut buf);
            for unit in utf16 {
                vec.push(*unit);
            }
        }
    }
    if !any_invalid {
        return Err(encoding_error());
    }
    if lossy.next() != Some('\0') {
        return Err(encoding_error());
    }

    // Validation succeeded.
    Ok(OsString::from_wide(&vec))
}

/// Slow path for `host_to_bytes`.
#[cold]
fn to_arf(units: &[u16]) -> String {
    let mut data = String::new();

    data.push('\u{feff}');

    for unit in decode_utf16(units.iter().cloned()) {
        match unit {
            Ok(c) => data.push(c),
            Err(_) => data.push('\u{fffd}'),
        }
    }

    data.push('\0');

    for unit in decode_utf16(units.iter().cloned()) {
        match unit {
            Ok(c) => data.push(c),
            Err(e) => {
                let bad = e.unpaired_surrogate();
                assert!(bad >= 0xd800 && bad <= 0xdfff);
                data.push('\0');
                data.push(std::char::from_u32(u32::from(bad - 0xd800)).unwrap());
            }
        }
    }

    data
}

#[cold]
fn encoding_error() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "invalid path string")
}

#[test]
fn utf16_inputs() {
    assert_eq!(
        String::from_utf16(&str_to_host("").unwrap().encode_wide().collect::<Vec<_>>()).unwrap(),
        ""
    );
    str_to_host("\0").unwrap_err();
    assert_eq!(
        String::from_utf16(&str_to_host("f").unwrap().encode_wide().collect::<Vec<_>>()).unwrap(),
        "f"
    );
    assert_eq!(
        String::from_utf16(
            &str_to_host("foo")
                .unwrap()
                .encode_wide()
                .collect::<Vec<_>>()
        )
        .unwrap(),
        "foo"
    );
    assert_eq!(
        String::from_utf16(
            &str_to_host("\u{fffd}")
                .unwrap()
                .encode_wide()
                .collect::<Vec<_>>()
        )
        .unwrap(),
        "\u{fffd}"
    );
    assert_eq!(
        String::from_utf16(
            &str_to_host("\u{fffd}foo")
                .unwrap()
                .encode_wide()
                .collect::<Vec<_>>()
        )
        .unwrap(),
        "\u{fffd}foo"
    );
    assert_eq!(
        String::from_utf16(
            &str_to_host("\u{feff}foo")
                .unwrap()
                .encode_wide()
                .collect::<Vec<_>>()
        )
        .unwrap(),
        "\u{feff}foo"
    );
}

#[test]
fn arf_inputs() {
    assert_eq!(
        str_to_host("\u{feff}hello\u{fffd}world\0hello\0\x05world")
            .unwrap()
            .encode_wide()
            .collect::<Vec<_>>(),
        [
            'h' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0xd805_u16, 'w' as u16,
            'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16
        ]
    );
    assert_eq!(
        str_to_host("\u{feff}hello\u{fffd}\0hello\0\x05")
            .unwrap()
            .encode_wide()
            .collect::<Vec<_>>(),
        ['h' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0xd805_u16]
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
fn valid_utf16() {
    assert_eq!(host_to_str(OsStr::new("")).unwrap(), "");
    assert_eq!(host_to_str(OsStr::new("foo")).unwrap(), "foo");
}

#[test]
fn not_utf16() {
    assert_eq!(
        host_to_str(&OsString::from_wide(&[0xd800_u16])).unwrap(),
        "\u{feff}\u{fffd}\0\0\u{0}"
    );
    assert_eq!(
        host_to_str(&OsString::from_wide(&[0xdfff_u16])).unwrap(),
        "\u{feff}\u{fffd}\0\0\u{7ff}"
    );
}

#[test]
fn round_trip() {
    assert_eq!(host_to_str(&bytes_to_host(b"").unwrap()).unwrap(), "");
    assert_eq!(
        host_to_str(&bytes_to_host(b"hello").unwrap()).unwrap(),
        "hello"
    );
    assert_eq!(
        str_to_host(&host_to_str(OsStr::new("hello")).unwrap()).unwrap(),
        OsStr::new("hello")
    );
    assert_eq!(
        str_to_host(&host_to_str(&OsString::from_wide(&[0x47_u16, 0xd800_u16, 0x48_u16])).unwrap())
            .unwrap(),
        OsString::from_wide(&[0x47_u16, 0xd800_u16, 0x48_u16])
    );
    assert_eq!(
        str_to_host(&host_to_str(&OsString::from_wide(&[0x49_u16, 0xdfff_u16, 0x50_u16])).unwrap())
            .unwrap(),
        OsString::from_wide(&[0x49_u16, 0xdfff_u16, 0x50_u16])
    );
    assert_eq!(
        str_to_host(&host_to_str(OsStr::new("")).unwrap()).unwrap(),
        OsStr::new("")
    );
}
