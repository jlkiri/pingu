use libc::c_void;
use pnet_sys::setsockopt;
use pnet_sys::IPPROTO_IP;
use pnet_sys::IP_HDRINCL;
use socket2::Socket;
use std::mem;

#[cfg(target_os = "linux")]
pub fn include_ip_header(socket: &Socket, value: bool) {
    use std::os::unix::prelude::AsRawFd;
    unsafe {
        if let -1 = setsockopt(
            socket.as_raw_fd(),
            IPPROTO_IP,
            IP_HDRINCL,
            &value as *const _ as *const c_void,
            mem::size_of::<*const c_void>() as u32,
        ) {
            panic!("setsockopt failed.");
        }
    }
}

#[cfg(target_os = "windows")]
pub fn include_ip_header(socket: &Socket, value: &bool) {
    use std::{os::windows::prelude::AsRawSocket};
    use libc::c_char;

    let opt = value as *const bool as *const c_char;

    unsafe {
        if let -1 = setsockopt(
            socket.as_raw_socket() as usize,
            IPPROTO_IP,
            IP_HDRINCL,
            opt,
            mem::size_of::<bool>() as i32
        ) {
            panic!("{}", std::io::Error::last_os_error());
        }
    }
}
