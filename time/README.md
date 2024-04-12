# time

A long-running program that constantly prints out the current time, writing in Go and using the WASI Preview 1 API.

## Build

```
$ GOOS=wasip1 GOARCH=wasm go build -o time.wasm main.go
```

## Reference

* https://go.dev/blog/wasi
