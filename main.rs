extern "Rust" {
    pub fn spec_extend(arr: &mut [u8; 10], ptr: *const u8, end: *const u8);
}

pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

#[inline]
pub fn extend_from_slice(arr: &mut [u8; 10], slice: &[u8]) {
    let ptr = slice.as_ptr();
    let end = unsafe { ptr.add(slice.len()) };
    assert!(slice.len() <= 10);
    unsafe { spec_extend(arr, ptr, end) }
}

#[inline(never)]
#[export_name = "encode"]
pub fn encode(version: Version, dst: &mut [u8; 10]) -> NonTrivialDrop {
    let has_drop = NonTrivialDrop;

    match version {
        Version::Http10 => extend_from_slice(dst, b"HTTP/1.0"),
        Version::Http11 => extend_from_slice(dst, b"HTTP/1.1"),
        Version::H2 => extend_from_slice(dst, b"HTTP/1.1"),
        _ => {}
    }

    has_drop
}

pub struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    #[inline(never)]
    #[export_name = "non_trivial_drop"]
    fn drop(&mut self) {
        core::hint::black_box(());
    }
}

fn main() {
    let mut dst = [0; 10];
    encode(Version::Http11, &mut dst);
    eprintln!("{dst:?}");
}
