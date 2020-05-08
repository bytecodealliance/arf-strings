#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate arf_strings;

use arf_strings::{PosixString, WasiString};
use std::ffi::CString;

fuzz_target!(|data: &[u8]| {
    if let Ok(cstr) = CString::new(data) {
        let wasi = WasiString::from_maybe_nonutf8_cstr(&cstr);
        let posix = PosixString::from_path_str(wasi.as_str()).unwrap();
        assert_eq!(
            cstr.as_c_str(),
            posix.as_cstr(),
            "\ndata: {:#x?}\nwasi: {}\nposix: {:#x?}\n",
            data,
            wasi.as_str(),
            posix.as_cstr()
        );
    }
});
