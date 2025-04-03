#!/bin/bash

set -ex

if [[ "$1" == "-c" ]]; then
  cargo clean
fi

EMCC_CFLAGS="-sUSE_GLFW=3"
EMCC_CFLAGS+=" -sMODULARIZE=1 -sEXPORT_ES6=1"
EMCC_CFLAGS+=" -sEXPORT_NAME=createRustyChess"
EMCC_CFLAGS+=" -sEXPORTED_FUNCTIONS=_start_game,_send_mouse_button_down,_send_mouse_button_released"
EMCC_CFLAGS+=" -sASYNCIFY"
EMCC_CFLAGS+=" --embed-file gui/assets"
EMCC_CFLAGS+=" -DPLATFORM_WEB"

CARGO_FLAG="--target=wasm32-unknown-emscripten --bin rusty-chess-gui"

EMCC_CFLAGS="$EMCC_CFLAGS" cargo build --release $CARGO_FLAG # NOTE: Cannot instantiate wasm when compiled in debug mode

cp ./target/wasm32-unknown-emscripten/release/{rusty-chess-gui.js,rusty_chess_gui.wasm} .

python3 -m http.server 3000
