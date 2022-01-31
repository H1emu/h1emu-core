# h1emu-core

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

