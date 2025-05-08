#!/bin/bash

rm -r build
mkdir build

rustc dep.rs -C opt-level=2 -C codegen-units=3 --out-dir ./build --target x86_64-apple-darwin

rustc foo.rs --out-dir ./build --target x86_64-apple-darwin --extern dep=./build/libdep.rlib '-Clinker=/Applications/Xcode 13.4.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld'

./build/foo
