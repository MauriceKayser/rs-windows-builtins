@echo off
title Run windows-builtins tests..

cd no_std
cargo +nightly run --release --target i686-pc-windows-msvc
cargo +nightly run --release --target x86_64-pc-windows-msvc
cd ..\std
cargo +nightly run --release --target i686-pc-windows-msvc
cargo +nightly run --release --target x86_64-pc-windows-msvc
cd ..

pause