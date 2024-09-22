default: run

build-wasm:
    cargo build --release --target wasm32-unknown-unknown --no-default-features
    wasm-bindgen --no-typescript --out-name bevy_game --out-dir wasm --target web target/wasm32-unknown-unknown/release/bottle_flip_jam_demo.wasm
    cp -r assets wasm/

run: build-wasm
    sfz -b 0.0.0.0 -p 8080 ./wasm