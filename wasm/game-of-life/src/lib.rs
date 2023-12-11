#[macro_use]
mod utils;

use std::fmt;
use utils::set_panic_hook;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead => '◻',
                    Cell::Alive => '◼',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        debug_assert_ne!(self.height, 0);
        debug_assert_ne!(self.width, 0);

        for row_diff in [self.height - 1, 0, 1] {
            for col_diff in [self.width - 1, 0, 1] {
                if row_diff == 0 && col_diff == 0 {
                    continue;
                }
                let new_row = ((u64::from(row) + u64::from(row_diff)) % u64::from(self.height)) as u32;
                let new_col = ((u64::from(column) + u64::from(col_diff)) % u64::from(self.width)) as u32;
                let neighbor_index = self.get_index(new_row, new_col);
                count += self.cells[neighbor_index] as u8;
            }
        }
        count
    }

    /// Get the dead and alive values of the entire universe.
    #[must_use] pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().copied() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn reset_cells(&mut self) {
        self.cells = self.cells.iter().map(|_| Cell::Dead).collect();
    }
}
/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let index = self.get_index(row, column);
        self.cells[index].toggle();
    }

    #[must_use] pub fn new(width: u32, height: u32) -> Universe {
        set_panic_hook();
        console_log!("Setting width to {width} and height to {height}.");

        assert_ne!(width, 0, "Universe width must be greater than zero!");
        assert_ne!(height, 0, "Universe height must be greater than zero!");

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    #[must_use] pub fn render(&self) -> String {
        self.to_string()
    }

    #[must_use] pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
