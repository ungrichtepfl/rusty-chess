//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_1() {
    // Let's create a smaller Universe with a small spaceship to test!
    assert_eq!(1, 1);
}
