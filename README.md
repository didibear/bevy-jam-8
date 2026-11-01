# Potion Market

Little game for the [Bevy Jam #8 - Alchemy](https://itch.io/jam/bevy-jam-8).

## Game

https://didibear.itch.io/potion-market

## Development

Sounds generated using [Chiptone](https://sfbgames.itch.io/chiptone)

## Build

0. Installs

```sh
cargo install -f wasm-bindgen-cli --version 0.2.130

sudo dnf install binaryen
cargo install wasm-opt --locked
```

1. Compile wasm app

```sh
cargo build --release --target wasm32-unknown-unknown
```

2. Create JS bindings

```sh
wasm-bindgen \
  --no-typescript --target web \
  --out-name bevy_game --out-dir ./wasm/target \
  ./target/wasm32-unknown-unknown/release/bevy-jam-8.wasm
```

3. Copy assets to the `wasm` folder

```sh
cp -r assets wasm
```

## Publish

- Start a local web server

```sh
basic-http-server wasm
```

- Publish to itch.io

```sh
zip wasm.zip wasm/**/*
```

## Licence

Code is licensed under MIT or Apache-2.0.
Assets are licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
