# printds

> [!WARNING]
> Windows is currently not supported.

## Build

Make sure you have `CUPS` installed.
On Linux based systems, you might also need `libcups2-dev` to link against `CUPS` during the build process.
To generate bindings, you will also need `libclang-dev`.
You can install both using your system's package manager, for example:

```sh
sudo apt install libcups2-dev libclang-dev
```

You will also need the [Rust toolchain](https://www.rust-lang.org/tools/install).
Then you can use Cargo to build the executable:

```sh
cargo build
```
