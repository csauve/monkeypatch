# Huragok
This is a stub project implementing a Windows DLL in Rust, intended to be used as a base for Halo modding experiments.

## Development
Assuming Rust has been setup already and compiling from windows, a DLL can be produced at `target/release/huragok.dll` using `cargo build --release`.

However, if cross-compiling from linux, first ensure the distro package `mingw-w64-gcc` is installed and the corresponding rust target:

```sh
rustup target add i686-pc-windows-gnu
```

Next, specify the target when building:

```sh
cargo build --release --target=i686-pc-windows-gnu
# produces target/i686-pc-windows-gnu/release/huragok.dll
```
