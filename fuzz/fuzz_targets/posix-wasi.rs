#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate arf_strings;

use arf_strings::{PosixString, WasiString};
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if !s.contains('\0') {
            let posix = PosixString::from_path_str(s).unwrap();
            let wasi = WasiString::from_maybe_nonutf8_cstr(posix.as_cstr());
            assert_eq!(
                s,
                wasi.as_str(),
                "\ndata: {:#x?}\nwasi: {}\nposix: {:#x?}\n",
                data,
                wasi.as_str(),
                posix.as_cstr()
            );
        }
    }
});
