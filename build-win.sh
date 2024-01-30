#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/big_band_orchestra.exe . &&
exec ./big_band_orchestra.exe "$@"