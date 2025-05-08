#!/bin/bash

rm -r build
mkdir build

export RUSTC="${RUSTC:-rustc}"

$RUSTC extend.rs --out-dir ./build --target x86_64-apple-darwin -C opt-level=1 -C codegen-units=1 --emit llvm-ir,link



$RUSTC main.rs -C opt-level=1 -C codegen-units=1 --crate-name=broken --out-dir ./build --target x86_64-apple-darwin --extern extend=./build/libextend.rlib '-Clinker=/Applications/Xcode 14.3.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld'
./build/broken

$RUSTC main.rs -C opt-level=1 -C codegen-units=1 --crate-name=working --out-dir ./build --target x86_64-apple-darwin --extern extend=./build/libextend.rlib
./build/working
