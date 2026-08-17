# ttf-parser as a WebAssembly module

## Build

```sh
rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release --manifest-path ../../c-api/Cargo.toml
cp ../../c-api/target/wasm32-unknown-unknown/release/ttfparser.wasm .
```

## Run

You can use any webserver that can serve `index.html`. Here is a Python example:

```sh
python -m http.server
```
