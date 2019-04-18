# Lolcat

The good ol' lolcat, now with fearless concurrency.


To install it, execute:

```bash
wapm install lolcat
```

![Look at deez colors](https://github.com/wapm-packages/lolcat/raw/master/screenshot.png)

Run lolcat on everything you like and you'll never have to wonder about the dullness of your life. See? It's that simple.

## License

Seriously? It's the [MIT license](LICENSE).


## Building from Source

First, you will need the WASI target installed in your Rust system:

```shell
rustup target add wasm32-unknown-wasi --toolchain nightly
```

Once WASI is available, you can build the WebAssembly binary by yourself with:

```shell
cargo +nightly build --release --target wasm32-unknown-wasi
```

This will create a new file located at `target/wasm32-unknown-wasi/release/cowsay.wasm`.

When the wasm file is created you can upload it to wapm or execute it with wasmer:

```shell
wapm publish
# OR
echo "Hey" | wasmer run  target/wasm32-unknown-wasi/release/lolcat.wasm
```

You can also build a native executable with

```shell
cargo build
```