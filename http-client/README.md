# http-client

This example is built using the WASI Preview 2 API.

## Build

First, install [cargo component](https://github.com/bytecodealliance/cargo-component):

```
cargo install cargo-component@0.11.0
```

Then execute the following command to compile it into a WASM program:

```
$ cargo component build
```

Or use `--release` option to build it in release mode:

```
$ cargo component build --release
```
