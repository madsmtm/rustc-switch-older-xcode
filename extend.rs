#![crate_type = "lib"]
#![no_std]

#[export_name = "spec_extend"]
pub unsafe fn spec_extend(arr: &mut [u8; 10], ptr: *const u8, end: *const u8) {
    let other: &[u8] = unsafe { core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize) };
    let count = other.len();
    unsafe { core::ptr::copy_nonoverlapping(other.as_ptr(), arr.as_mut_ptr(), count) };
}
