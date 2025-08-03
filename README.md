# printrs 🖨️

> [!WARNING]
> Windows is currently not supported.

## Requirements

Make sure you have `CUPS` installed.
On Linux based systems, you might also need `libcups2-dev` to link against `CUPS` during the build process.
To generate bindings, you will also need `libclang-dev`.
You can install both using your system's package manager, for example:

```sh
sudo apt install cups libcups2-dev libclang-dev
```

You will also need the [Rust toolchain](https://www.rust-lang.org/tools/install).

## Build

Build the executable with:

```sh
cargo build -r
```

and run with:

```sh
./target/release/printrs --help
```

Alternatively, build and run directly with one command:

```sh
cargo run -q -- --help
```
