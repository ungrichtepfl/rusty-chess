//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rusty_chess_wasm::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.reset_cells();
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.reset_cells();
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
