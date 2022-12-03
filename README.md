# h1emu-core [![npm version](http://img.shields.io/npm/v/h1emu-core.svg?style=flat)](https://npmjs.org/package/h1emu-core "View this project on npm")

Utility library used in h1emu.

## Build

### Needed dependencies

* Rust toolchain : https://www.rust-lang.org/tools/install
* wasm-pack : `cargo install wasm-pack`

### Build the wasm module

run `wasm-pack build -t nodejs` to build the wasm module in the /pkg folder


## misc 

### run unit tests

run `cargo test` to see if your change broke nothing.

### run benchmarks

run `cargo bench`.

