# monkeypatch
This project implements a small proof-of-concept client mod for Halo: CE which patches game code at runtime. It compiles to a `strings.dll` which can be dropped into the games's installation directory and gets loaded by the game. This is intended to be a base for experiments and personal use rather than a fully-featured client mod, like Chimera, HAC2, OpenSauce, or Vulpes.

The project currently just increases the FOV from 70 to 90 degrees.

## Development
Firstly, install the i686 toolchain:
* On Windows: `rustup target add i686-pc-windows-msvc`
* If cross-compiling from linux, ensure `mingw-w64-gcc` packages are installed and run `rustup target add i686-pc-windows-gnu`

The project can now be compiled with `cargo build --release --target=<toolchain>`. The binary will be found in `target/<toolchain>/release/strings.dll` and can be copied into the Halo installation.
