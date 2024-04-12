# hello

This Hello World program is compiled to a WebAssembly module using the WASI Preview 1 API.

## Build

```
$ cargo build
   Compiling hello v0.0.0 (/iawia002/wasi-examples/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
```

The WASM file is generated under the path `target/wasm32-wasi/debug/hello.wasm`.
