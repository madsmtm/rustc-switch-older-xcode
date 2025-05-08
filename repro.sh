#!/bin/bash

rm -r build
mkdir build

export RUSTC="${RUSTC:-rustc}"

$RUSTC custom_std.rs --out-dir ./build --target x86_64-apple-darwin -C opt-level=1 -C codegen-units=1 --emit llvm-ir,link

$RUSTC dep.rs --out-dir ./build --target x86_64-apple-darwin --extern custom_std=./build/libcustom_std.rlib -C opt-level=1 -C codegen-units=1 -Cembed-bitcode=no --emit llvm-ir,link



$RUSTC foo.rs --crate-name=foo_broken --out-dir ./build --target x86_64-apple-darwin -L ./build --extern custom_std=./build/libcustom_std.rlib --extern dep=./build/libdep.rlib '-Clinker=/Applications/Xcode 14.3.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld'
./build/foo_broken

$RUSTC foo.rs --crate-name=foo_working --out-dir ./build --target x86_64-apple-darwin -L ./build --extern custom_std=./build/libcustom_std.rlib --extern dep=./build/libdep.rlib
./build/foo_working
