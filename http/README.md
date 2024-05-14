# http

This example is built using the [WASI Preview 2 API](https://github.com/WebAssembly/wasi-http).

## Build

First, install [cargo component](https://github.com/bytecodealliance/cargo-component):

```
cargo install cargo-component@0.11.0
```

Then execute the following command to compile it into a WASM program:

```
$ cargo component build
   Compiling http v0.0.0 (/iawia002/wasi-examples/http)
    Finished dev [unoptimized + debuginfo] target(s) in 1.25s
    Creating component target/wasm32-wasi/debug/http.wasm
```

Or use `--release` option to build it in release mode:

```
$ cargo component build --release
   Compiling http v0.0.0 (/iawia002/wasi-examples/http)
    Finished release [optimized] target(s) in 12.67s
    Creating component target/wasm32-wasi/release/http.wasm
```
