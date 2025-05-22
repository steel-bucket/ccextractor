use lib_ccxr::info;
use crate::bindings::ccx_demuxer;

#[no_mangle]
pub unsafe extern "C" fn ccxr_demuxer_close(ctx: *mut ccx_demuxer) {
    if ctx.is_null() {
        return;
    }
    info!("infd check in rust 1 {} ", (*ctx).infd);
    (*ctx).infd = -1;
    (*ctx).past = -1;
    info!("infd check in rust 2 {} ", (*ctx).infd);
}