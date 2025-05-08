#![no_main]

#[inline(never)]
#[no_mangle]
pub unsafe fn slice_len_from_ptr_end(ptr: *const u8, end: *const u8) -> usize {
    core::hint::black_box(unsafe { end.offset_from(ptr) as usize })
}

#[inline]
pub fn slice_len(slice: &[u8]) -> usize {
    let ptr = slice.as_ptr();
    let end = unsafe { ptr.add(slice.len()) };
    unsafe { slice_len_from_ptr_end(ptr, end) }
}

pub struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    #[inline(never)]
    #[export_name = "non_trivial_drop"]
    fn drop(&mut self) {
        core::hint::black_box(());
    }
}

pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

#[no_mangle]
fn broken() -> usize {
    let _has_drop = NonTrivialDrop;

    match core::hint::black_box(Version::Http11) {
        Version::Http10 => slice_len(b"HTTP/1.0"),
        Version::Http11 => slice_len(b"HTTP/1.1"),
        Version::H2 => slice_len(b"HTTP/1.1"),
        _ => 0,
    }
}

#[no_mangle]
extern "C" fn main(/* argv, argc */) -> core::ffi::c_int {
    broken() as _
}
