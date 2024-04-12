# wasi-examples

Bunch of WebAssembly app examples using WASI API.

## Overview

All examples are built using the [WASI Preview 2 API](https://github.com/WebAssembly/WASI/tree/main/preview2), with each folder representing a separate sample project.

## Build

First, install [cargo component](https://github.com/bytecodealliance/cargo-component):

```text
cargo install cargo-component
```

Then navigate to any other example directory and execute the following command to compile it into a wasm program:

```text
$ cargo component build
   Compiling http v0.1.0 (/iawia002/wasi-examples/http)
    Finished dev [unoptimized + debuginfo] target(s) in 1.25s
    Creating component target/wasm32-wasi/debug/http.wasm
```

Or use `--release` option to build it in release mode:

```text
$ cargo component build --release
   Compiling http v0.1.0 (/iawia002/wasi-examples/http)
    Finished release [optimized] target(s) in 12.67s
    Creating component target/wasm32-wasi/release/http.wasm
```

## Reference

* https://github.com/WebAssembly/wasi-cli
* https://github.com/WebAssembly/wasi-http
* https://github.com/sunfishcode/hello-wasi-http
