#!/bin/sh

# build release for windows
cross build --release --target=x86_64-pc-windows-gnu
md5sum target/x86_64-pc-windows-gnu/release/command-line-apps-in-rust.exe