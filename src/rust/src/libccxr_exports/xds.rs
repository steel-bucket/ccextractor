use crate::xds::common_types::*;
use lib_ccxr::common::options::CommonTimingCtx;

#[no_mangle]
pub unsafe extern "C" fn ccxr_ccx_decoders_xds_init_library(
    timing: *mut CommonTimingCtx,
    xds_write_to_file: i32,
) -> *mut CcxDecodersXdsContext {
    if timing.is_null() {
        return std::ptr::null_mut();
    }
    let ctx = CcxDecodersXdsContext::new(timing, xds_write_to_file);
    Box::into_raw(ctx)
}

#[no_mangle]
pub unsafe extern "C" fn ccxr_free_ccx_decoders_xds_context(ptr: *mut CcxDecodersXdsContext) {
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}