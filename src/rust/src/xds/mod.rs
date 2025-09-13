use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};
use crate::avc::FromCType;
use crate::bindings::*;
use crate::xds::common_types::{copy_xds_context_from_rust_to_c, CcxDecodersXdsContext};
use crate::xds::core::write_xds_string;

pub mod core;
pub mod common_types;
pub mod common_constants;
//int ccxr_write_xds_string(struct cc_subtitle *sub, struct ccx_decoders_xds_context *ctx, char *p, size_t len, LLONG ts_start_of_xds);
#[no_mangle]
pub unsafe extern "C" fn ccxr_write_xds_string(
    sub: *mut cc_subtitle,
    ctx: *mut ccx_decoders_xds_context,
    p: *const c_char,
    len: c_uint,
    ts_start_of_xds: i64,
) -> c_int {
    if sub.is_null() || ctx.is_null() || p.is_null() {
        return -1;
    }
    let p_str = match CStr::from_ptr(p).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -1,
    };
    let mut rust_ctx = CcxDecodersXdsContext::from_ctype(*ctx).unwrap();
    let output = match write_xds_string(&mut *sub, &mut rust_ctx, p_str, len as usize, ts_start_of_xds) {
        Ok(_) => 0,
        Err(_) => -1,
    };
    copy_xds_context_from_rust_to_c(ctx, &rust_ctx);
    output
}