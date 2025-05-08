#![crate_type = "lib"]

#[export_name = "dbg_slice_from_ptr_end"]
pub unsafe extern "C-unwind" fn dbg_slice_from_ptr_end(ptr: *const u8, end: *const u8) {
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize) };
    eprintln!("{slice:?}");
}
