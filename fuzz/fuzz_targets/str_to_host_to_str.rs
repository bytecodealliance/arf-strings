#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate arf_strings;

use arf_strings::{host_os_str_to_str, str_to_host};
#[cfg(not(windows))]
use std::ffi::CStr;
use std::{ffi::OsStr, str};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if !s.contains('\0') {
            let host = str_to_host(s).unwrap();
            let host_encoding = to_host_encoding(&host);
            let result = host_os_str_to_str(host_encoding).unwrap();
            assert_eq!(
                s, result,
                "\ndata: {:#x?}\nresult: {}\nhost: {:#x?}\n",
                data, result, host
            );
        }
    }
});

#[cfg(not(windows))]
fn to_host_encoding(host: &CStr) -> &OsStr {
    use std::os::unix::ffi::OsStrExt;
    OsStr::from_bytes(host.to_bytes())
}
