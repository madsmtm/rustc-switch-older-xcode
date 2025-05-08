#!/bin/bash

rm -r build
mkdir build

export RUSTC="${RUSTC:-rustc}"

$RUSTC dependency.rs --out-dir ./build --target x86_64-apple-darwin --emit obj



$RUSTC main.rs -C opt-level=1 -C codegen-units=1 --out-dir ./build --target x86_64-apple-darwin -Clink-arg=./build/dependency.o --crate-name=broken -Clink-arg=-ld_classic
./build/broken

$RUSTC main.rs -C opt-level=1 -C codegen-units=1 --out-dir ./build --target x86_64-apple-darwin -Clink-arg=./build/dependency.o --crate-name=working
./build/working
