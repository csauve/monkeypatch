# monkeypatch
This is a small client mod for `halo_tag_test.exe` (aka Standalone) in the Halo CE Mod Tools, which just increases the FOV. This is intended to be a base for experiments and personal use rather than a fully-featured client mod.

**WARNING: you must temporarily remove this mod to use an Xbox controller in Sapien for recorded animations!**

## Installation
Just copy `xinput1_4.dll` into the mod tools installation folder.

## Development
Firstly, install the i686 toolchain:
* On Windows: `rustup target add i686-pc-windows-msvc`
* If cross-compiling from linux, ensure `mingw-w64-gcc` packages are installed and run `rustup target add i686-pc-windows-gnu`

The project can now be compiled with `cargo build --release --target=<toolchain>`. The binary will be found in `target/<toolchain>/release/xinput1_4.dll`.
