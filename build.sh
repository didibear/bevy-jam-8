# Build
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen \
  --no-typescript --target web \
  --out-name bevy_game --out-dir ./wasm/target \
  ./target/wasm32-unknown-unknown/release/bevy-jam-8.wasm
wasm-opt -Oz --all-features \
  -o ./wasm/target/bevy_game_bg.wasm ./wasm/target/bevy_game_bg.wasm

# Bundle into the wasm dir
cp -r assets wasm
(cd wasm && zip -r ../wasm.zip .)

# Start it locally
basic-http-server wasm
