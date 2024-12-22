cargo build --release --target wasm32-unknown-unknown

# change `bevy_starter` below as appropriate
# you may need to `cargo update` if the below fails because of a schema error
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "genuary_2025" \
    ./target/wasm32-unknown-unknown/release/genuary_2025.wasm