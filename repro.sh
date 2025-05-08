#!/bin/bash

rm -r build
mkdir build

export RUSTC="${RUSTC:-rustc}"


$RUSTC main.rs -C opt-level=1 -C codegen-units=1 --out-dir ./build --target x86_64-apple-darwin --emit=llvm-ir,link --crate-name=broken -Clink-arg=-ld_classic
./build/broken
echo "Status code: $?"

$RUSTC main.rs -C opt-level=1 -C codegen-units=1 --out-dir ./build --target x86_64-apple-darwin --emit=llvm-ir,link --crate-name=working
./build/working
echo "Status code: $?"
