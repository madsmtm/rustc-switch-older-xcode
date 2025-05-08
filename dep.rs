#![crate_type = "lib"]

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash, Debug)]
pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

pub fn encode(version: &Version, dst: &mut Vec<u8>) -> HasDrop {
    let has_drop = HasDrop;

    match version {
        Version::Http10 => dst.extend_from_slice(b"HTTP/1.0"),
        Version::Http11 => dst.extend_from_slice(b"HTTP/1.1"),
        Version::H2 => dst.extend_from_slice(b"HTTP/1.1"),
        _ => std::process::abort(),
    }

    has_drop
}

/// Something with a non-trivial Drop.
pub struct HasDrop;

impl Drop for HasDrop {
    #[inline(never)]
    fn drop(&mut self) {
        eprintln!("drop");
    }
}
