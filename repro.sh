#!/bin/bash

rm -r build
mkdir build

rustc dep.rs --out-dir ./build --target x86_64-apple-darwin -C opt-level=1 -C codegen-units=1 -Cembed-bitcode=no

rustc foo.rs --out-dir ./build --target x86_64-apple-darwin --extern dep=./build/libdep.rlib '-Clinker=/Applications/Xcode 14.3.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld'

./build/foo
