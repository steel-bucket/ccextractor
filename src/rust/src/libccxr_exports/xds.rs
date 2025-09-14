use crate::xds::common_types::*;
use lib_ccxr::common::options::CommonTimingCtx;
use lib_ccxr::time::TimingContext;


#[no_mangle]
pub unsafe extern "C" fn ccxr_free_ccx_decoders_xds_context(ptr: *mut CcxDecodersXdsContext) {
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}