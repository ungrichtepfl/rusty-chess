#[macro_use]
mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use rusty_chess_core::game::{Game, UserInput, UserOutput};

#[wasm_bindgen]
pub struct ChessGame {
    game: Game,
}

#[wasm_bindgen]
extern "C" {
    fn alert(msg: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct UserOutputWrapper(UserOutput);

#[wasm_bindgen]
#[derive(Debug)]
pub struct PositionWrapper(pub char, pub char);

#[wasm_bindgen]
impl UserOutputWrapper {
    pub fn is_check_mate(&self) -> bool {
        match self.0 {
            UserOutput::CheckMate => true,
            _ => false,
        }
    }

    pub fn is_stale_mate(&self) -> bool {
        match self.0 {
            UserOutput::StaleMate => true,
            _ => false,
        }
    }
    pub fn is_invalid_move(&self) -> bool {
        match self.0 {
            UserOutput::InvalidMove => true,
            _ => false,
        }
    }
    pub fn is_promotion(&self) -> bool {
        match self.0 {
            UserOutput::Promotion(_) => true,
            _ => false,
        }
    }
    pub fn promotion_pos(&self) -> Option<PositionWrapper> {
        match self.0 {
            UserOutput::Promotion(pos) => Some(PositionWrapper(pos.0, pos.1)),
            _ => None,
        }
    }
    pub fn is_draw(&self) -> bool {
        match self.0 {
            UserOutput::Draw => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self.0 {
            UserOutput::CheckMate => "CheckMate".to_string(),
            UserOutput::StaleMate => "StaleMate".to_string(),
            UserOutput::InvalidMove => "InvalidMove".to_string(),
            UserOutput::Promotion(pos) => format!("Promotion ({},{})", pos.0, pos.1),
            UserOutput::Draw => "Draw".to_string(),
        }
    }
}

#[wasm_bindgen]
impl ChessGame {
    pub fn new() -> ChessGame {
        set_panic_hook();
        let game = Game::new();

        ChessGame { game }
    }

    pub fn play_move(
        &mut self,
        from1: char,
        from2: char,
        to1: char,
        to2: char,
    ) -> Option<UserOutputWrapper> {
        self.game
            .process_input(&UserInput::Move((from1, from2).into(), (to1, to2).into()))
            .map(|x| UserOutputWrapper(x))
    }

    pub fn play_randomly_aggressive(&mut self) -> Option<UserOutputWrapper> {
        let possible_moves = self.game.get_all_currently_valid_moves();
        if possible_moves.is_empty() {
            console_log!("Something went wrong. Function was probably called after check mate or stale mate.");
            return None;
        }
        let move_to_play = match possible_moves.iter().find(|mv| mv.captured_piece.is_some()) {
            Some(mv) => mv,
            None => {
                let random_index = (js_sys::Math::random() * (possible_moves.len() as f64 - 1.0))
                    as usize;
                &possible_moves[random_index]
            },
        };
        console_log!("Move played {move_to_play}!");

        self.game
            .process_input(&UserInput::Move(move_to_play.from, move_to_play.to))
            .map(|x| UserOutputWrapper(x))
    }

    pub fn render(&self) -> String {
        self.game.to_string()
    }
}
