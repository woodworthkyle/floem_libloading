#!/usr/bin/bash

export RUST_BACKTRACE=1
export RUST_LOG=debug
#cargo build --release --target x86_64-pc-windows-gnu
#./target/x86_64-pc-windows-gnu/release/host.exe
cargo build --target x86_64-pc-windows-gnu
#./target/x86_64-pc-windows-gnu/debug/host.exe
#cargo build 
#./target/debug/host

#cargo build
#./target/debug/host