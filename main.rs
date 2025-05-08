#![no_main]

extern "C-unwind" {
    fn dbg_slice_from_ptr_end(ptr: *const u8, end: *const u8);
}

pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

#[inline]
pub fn dbg_slice(slice: &[u8]) {
    let ptr = slice.as_ptr();
    let end = unsafe { ptr.add(slice.len()) };
    unsafe { dbg_slice_from_ptr_end(ptr, end) }
}

#[export_name = "main"]
extern "C" fn main(/* argv, argc */) -> core::ffi::c_int {
    let _has_drop = NonTrivialDrop;

    match core::hint::black_box(Version::Http11) {
        Version::Http10 => dbg_slice(b"HTTP/1.0"),
        Version::Http11 => dbg_slice(b"HTTP/1.1"),
        Version::H2 => dbg_slice(b"HTTP/1.1"),
        _ => {}
    }

    0
}

pub struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    #[inline(never)]
    #[export_name = "non_trivial_drop"]
    fn drop(&mut self) {
        core::hint::black_box(());
    }
}
