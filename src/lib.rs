mod posix_string;
mod wasi_string;

pub use posix_string::PosixString;
pub use wasi_string::WasiString;

#[test]
fn round_trip() {
    use std::ffi::CStr;
    assert_eq!(
        WasiString::from_maybe_nonutf8_c_str(PosixString::from_path_bytes(b"").unwrap().as_c_str())
            .as_str(),
        ""
    );
    assert_eq!(
        WasiString::from_maybe_nonutf8_c_str(
            PosixString::from_path_bytes(b"hello").unwrap().as_c_str()
        )
        .as_str(),
        "hello"
    );
    assert_eq!(
        PosixString::from_path_str(
            WasiString::from_maybe_nonutf8_c_str(CStr::from_bytes_with_nul(b"hello\0").unwrap())
                .as_str()
        )
        .unwrap()
        .as_c_str(),
        CStr::from_bytes_with_nul(b"hello\0").unwrap()
    );
    assert_eq!(
        PosixString::from_path_str(
            WasiString::from_maybe_nonutf8_c_str(
                CStr::from_bytes_with_nul(b"h\xc0ello\xc1\0").unwrap()
            )
            .as_str()
        )
        .unwrap()
        .as_c_str(),
        CStr::from_bytes_with_nul(b"h\xc0ello\xc1\0").unwrap()
    );
    assert_eq!(
        PosixString::from_path_str(
            WasiString::from_maybe_nonutf8_c_str(CStr::from_bytes_with_nul(b"\xf5\xff\0").unwrap())
                .as_str()
        )
        .unwrap()
        .as_c_str(),
        CStr::from_bytes_with_nul(b"\xf5\xff\0").unwrap()
    );
    assert_eq!(
        PosixString::from_path_str(
            WasiString::from_maybe_nonutf8_c_str(CStr::from_bytes_with_nul(b"\0").unwrap()).as_str()
        )
        .unwrap()
        .as_c_str(),
        CStr::from_bytes_with_nul(b"\0").unwrap()
    );
    assert_eq!(
        PosixString::from_path_str(
            WasiString::from_maybe_nonutf8_c_str(CStr::from_bytes_with_nul(b"\xe6\x96\0").unwrap())
                .as_str()
        )
        .unwrap()
        .as_c_str(),
        CStr::from_bytes_with_nul(b"\xe6\x96\0").unwrap()
    );
}
