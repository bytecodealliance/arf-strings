#[cfg(not(windows))]
mod rustix;
#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
pub use crate::rustix::{
    bytes_to_host, host_c_str_to_bytes, host_c_str_to_str, host_os_str_to_bytes,
    host_os_str_to_str, str_to_host,
};
#[cfg(windows)]
pub use windows::{bytes_to_host, host_to_bytes, host_to_str, str_to_host};
