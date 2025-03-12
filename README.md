# rusty-chess

This is a simple chess game written in Rust. It is a work in progress and
contains different submodules for the different parts of the game.

- [core](core/README.md) contains the game logic
- [gui](gui/README.md) contains the graphical user interface for playing the game
- [cli](cli/README.md) contains the command line interface
- [wasm](wasm/README.md) contains a wasm playground for the game

## Usage

Install the [rust compiler](https://www.rust-lang.org/learn/get-started), change
directory into the repo and run:

```shell
cargo run
```

This will start the [gui](gui/README.md). You can also play the game using the
[cli](cli/README.md) when you run:

```shell
cd cli && cargo run
```

## Build for WEB

To build for web run:

```shell
./build_web.sh
```

You need to have the [emscripten](https://emscripten.org/) toolchain installed.
The compilation was tested using emcc version `4.0.2 (7591f1c5ea0adf6f4293cfba2995ee9700aa0d93)`.

## Todo

- [x] create modules
- [x] GUI
- [x] docs
- [ ] smart chess computer
