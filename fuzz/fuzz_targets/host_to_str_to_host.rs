#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate arf_strings;

use arf_strings::{host_os_str_to_str, str_to_host};
use std::ffi::OsStr;

fuzz_target!(|data: &[u8]| {
    if data.iter().contains(b'\0') {
        return;
    }

    let host_encoding = to_host_encoding(data);
    let intermediate = host_os_str_to_str(&host_encoding).unwrap();
    let host = str_to_host(&intermediate)
        .expect(&format!("data={:?} str_to_host({:?}", data, intermediate));
    assert_eq!(
        host_encoding,
        to_host_encoding(host.as_bytes()),
        "\ndata: {:#x?}\nintermediate: {}\nhost: {:#x?}\n",
        data,
        intermediate,
        host
    );
});

#[cfg(not(windows))]
fn to_host_encoding(data: &[u8]) -> &OsStr {
    use std::os::unix::ffi::OsStrExt;
    OsStr::from_bytes(data)
}
