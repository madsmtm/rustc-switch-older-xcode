#![crate_type = "lib"]

pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

pub fn encode(version: Version, dst: &mut Vec<u8>) -> NonTrivialDrop {
    let has_drop = NonTrivialDrop;

    match version {
        Version::Http10 => dst.extend_from_slice(b"HTTP/1.0"),
        Version::Http11 => dst.extend_from_slice(b"HTTP/1.1"),
        Version::H2 => dst.extend_from_slice(b"HTTP/1.1"),
        _ => {}
    }

    has_drop
}

pub struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    #[inline(never)]
    #[export_name = "non_trivial_drop"]
    fn drop(&mut self) {
        let _ = Box::new(0);
    }
}
