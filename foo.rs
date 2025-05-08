extern crate dep;

use dep::{Version, encode};

fn main() {
    let mut dst = [0; 10];
    encode(Version::Http11, &mut dst);
    eprintln!("{dst:?}");
}
