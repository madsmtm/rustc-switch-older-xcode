extern crate dep;

use dep::{Version, encode};

fn main() {
    let mut dst = Vec::with_capacity(100);
    encode(&Version::Http11, &mut dst);
    let res = std::str::from_utf8(&dst).unwrap();
    eprintln!("{res:?}");
}
