# paperjet 🖨️

> [!WARNING]
> Windows is currently not supported.

## Requirements

You will need the [Rust toolchain](https://www.rust-lang.org/tools/install).

On Unix and macOS, you will need CUPS, which most likely is already installed.
For development, you will also need `libcups2-dev` and `liblcang-dev`.
You can install all of the above using your system's package manager, for example:

```sh
sudo apt install cups libcups2-dev libclang-dev
```

For PDF transformations, PDFium is currently used, and can be linked dynamically after building
the executable.
You can download the PDFium binary [here](https://github.com/bblanchon/pdfium-binaries/releases)
and put it in `target/debug` or `target/release` after the Rust executable is built.

Switching to static linking is planned, so that there is no manual downloading required.

## Build

Build the executable with:

```sh
cargo build -r
```

and run with:

```sh
./target/release/paperjet --help
```

Alternatively, build and run directly with one command:

```sh
cargo run -q -- --help
```
