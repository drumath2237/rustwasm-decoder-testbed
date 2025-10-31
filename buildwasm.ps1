cargo build --target wasm32-unknown-unknown --release

# rm -r ./wasm-test/src/wasm

wasm-bindgen ./target/wasm32-unknown-unknown/debug/rustwasm_decoder_testbed.wasm --out-dir ./wasm-test/src/wasm --out-name sog-wasm

wasm-opt ./wasm-test/src/wasm/sog-wasm_bg.wasm -o ./wasm-test/src/wasm/sog-wasm_bg.opt.wasm -O

mv -Force ./wasm-test/src/wasm/sog-wasm_bg.opt.wasm ./wasm-test/src/wasm/sog-wasm_bg.wasm