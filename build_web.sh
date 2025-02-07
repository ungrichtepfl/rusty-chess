#!/bin/bash

set -ex

if [[ "$1" == "-c" ]]; then
  cargo clean
fi

EMCC_CFLAGS="-sUSE_GLFW=3"
# EMCC_CFLAGS+=" -sWASM=1"
# EMCC_CFLAGS+=" -sGL_ENABLE_GET_PROC_ADDRESS"
EMCC_CFLAGS+=" -sASYNCIFY"
EMCC_CFLAGS+=" --embed-file gui/assets"
EMCC_CFLAGS+=" -DPLATFORM_WEB"

CARGO_FLAG="--target=wasm32-unknown-emscripten --bin rusty-chess-gui"

EMCC_CFLAGS="$EMCC_CFLAGS" cargo build --release $CARGO_FLAG # NOTE: Cannot instantiate wasm when compiled in debug mode

cp ./target/wasm32-unknown-emscripten/release/{rusty-chess-gui.js,rusty_chess_gui.wasm} .

serve -s
