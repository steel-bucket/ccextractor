use crate::bindings::{
    cc_subtitle, connect_to_srv, net_check_conn, net_send_cc, net_send_epg, net_send_header,
    net_tcp_read, net_udp_read, start_tcp_srv, start_upd_srv,
};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_void};

/// C-FFI wrapper for `connect_to_srv`.
///
/// # Safety
/// All pointers must be valid C strings (or null).
#[no_mangle]
pub unsafe extern "C" fn ccxr_connect_to_srv(
    addr: *const c_char,
    port: *const c_char,
    cc_desc: *const c_char,
    pwd: *const c_char,
) {
    connect_to_srv(addr, port, cc_desc, pwd);
}

/// C-FFI wrapper for `net_send_header`.
///
/// # Safety
/// `data` must be valid for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn ccxr_net_send_header(data: *const c_uchar, len: usize) {
    net_send_header(data, len);
}

/// C-FFI wrapper for `net_send_cc`.
///
/// # Safety
/// `data` must be valid for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn ccxr_net_send_cc(
    data: *const c_uchar,
    len: usize,
    private_data: *const c_void,
    sub: *const cc_subtitle,
) -> c_int {
    let len_i32 = if len > c_int::MAX as usize {
        c_int::MAX
    } else {
        len as c_int
    };
    net_send_cc(data, len_i32, private_data as *mut c_void, sub as *mut cc_subtitle)
}

/// C-FFI wrapper for `net_check_conn`.
#[no_mangle]
pub unsafe extern "C" fn ccxr_net_check_conn() {
    net_check_conn();
}

/// C-FFI wrapper for `net_send_epg`.
///
/// # Safety
/// All pointers must be valid C strings (or null).
#[no_mangle]
pub unsafe extern "C" fn ccxr_net_send_epg(
    start: *const c_char,
    stop: *const c_char,
    title: *const c_char,
    desc: *const c_char,
    lang: *const c_char,
    category: *const c_char,
) {
    net_send_epg(start, stop, title, desc, lang, category);
}

/// C-FFI wrapper for `net_tcp_read`.
///
/// # Safety
/// `buffer` must be valid for `length` bytes.
#[no_mangle]
pub unsafe extern "C" fn ccxr_net_tcp_read(
    socket: c_int,
    buffer: *mut c_void,
    length: usize,
) -> c_int {
    net_tcp_read(socket, buffer, length)
}

/// C-FFI wrapper for `net_udp_read`.
///
/// # Safety
/// `buffer` must be valid for `length` bytes.
#[no_mangle]
pub unsafe extern "C" fn ccxr_net_udp_read(
    socket: c_int,
    buffer: *mut c_void,
    length: usize,
    src_str: *const c_char,
    addr_str: *const c_char,
) -> c_int {
    net_udp_read(socket, buffer, length, src_str, addr_str)
}

/// C-FFI wrapper for `start_tcp_srv`.
///
/// # Safety
/// `port` and `pwd` must be valid C strings (or null).
#[no_mangle]
pub unsafe extern "C" fn ccxr_start_tcp_srv(port: *const c_char, pwd: *const c_char) -> c_int {
    start_tcp_srv(port, pwd)
}

/// C-FFI wrapper for `start_upd_srv`.
///
/// # Safety
/// `src` and `addr` must be valid C strings (or null).
#[no_mangle]
pub unsafe extern "C" fn ccxr_start_udp_srv(
    src: *const c_char,
    addr: *const c_char,
    port: c_uint,
) -> c_int {
    start_upd_srv(src, addr, port)
}
