cargo build --release --target wasm32-unknown-unknown

# change `bevy_starter` below as appropriate
# you may need to `cargo update` if the below fails because of a schema error
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "bevy_starter" \
    ./target/wasm32-unknown-unknown/debug/bevy_starter.wasm
    #./target/wasm32-unknown-unknown/release/bevy_starter.wasm