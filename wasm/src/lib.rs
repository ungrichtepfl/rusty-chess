#[macro_use]
mod utils;

use std::fmt::{self, Display, Formatter};

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use rusty_chess_core::game::{Color, Game, PieceType, UserInput, UserOutput};

// Canvas in wasm
// https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html

#[wasm_bindgen]
pub struct ChessGame {
    game: Game,
    game_board: [Piece; 64],
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Empty = 0,
    PawnWhite = 1,
    PawnBlack = 2,
    KnightWhite = 3,
    KnightBlack = 4,
    BishopWhite = 5,
    BishopBlack = 6,
    RookWhite = 7,
    RookBlack = 8,
    QueenWhite = 9,
    QueenBlack = 10,
    KingWhite = 11,
    KingBlack = 12,
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
    #[must_use] pub fn is_check_mate(&self) -> bool {
        matches!(self.0, UserOutput::CheckMate)
    }

    #[must_use] pub fn is_stale_mate(&self) -> bool {
        matches!(self.0, UserOutput::StaleMate)
    }
    #[must_use] pub fn is_invalid_move(&self) -> bool {
        matches!(self.0, UserOutput::InvalidMove)
    }
    #[must_use] pub fn is_promotion(&self) -> bool {
        matches!(self.0, UserOutput::Promotion(_))
    }
    #[must_use] pub fn promotion_pos(&self) -> Option<PositionWrapper> {
        match self.0 {
            UserOutput::Promotion(pos) => Some(PositionWrapper(pos.0, pos.1)),
            _ => None,
        }
    }
    #[must_use] pub fn is_draw(&self) -> bool {
        matches!(self.0, UserOutput::Draw)
    }
}

impl Display for UserOutputWrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self.0 {
            UserOutput::CheckMate => "CheckMate".to_string(),
            UserOutput::StaleMate => "StaleMate".to_string(),
            UserOutput::InvalidMove => "InvalidMove".to_string(),
            UserOutput::Promotion(pos) => format!("Promotion ({},{})", pos.0, pos.1),
            UserOutput::Draw => "Draw".to_string(),
        };
        write!(f, "{s}")
    }
}
impl ChessGame {
    fn update_game_board(&mut self) {
        for (i, piece) in self.game.board.iter().enumerate() {
            if let Some(piece) = &piece {
                let piece = match (&piece.piece_type, &piece.color) {
                    (PieceType::Pawn, Color::White) => Piece::PawnWhite,
                    (PieceType::Pawn, Color::Black) => Piece::PawnBlack,
                    (PieceType::Knight, Color::White) => Piece::KnightWhite,
                    (PieceType::Knight, Color::Black) => Piece::KnightBlack,
                    (PieceType::Bishop, Color::White) => Piece::BishopWhite,
                    (PieceType::Bishop, Color::Black) => Piece::BishopBlack,
                    (PieceType::Rook, Color::White) => Piece::RookWhite,
                    (PieceType::Rook, Color::Black) => Piece::RookBlack,
                    (PieceType::Queen, Color::White) => Piece::QueenWhite,
                    (PieceType::Queen, Color::Black) => Piece::QueenBlack,
                    (PieceType::King, Color::White) => Piece::KingWhite,
                    (PieceType::King, Color::Black) => Piece::KingBlack,
                };
                self.game_board[i] = piece;
            } else {
                self.game_board[i] = Piece::Empty;
            }
        }
    }

    #[allow(dead_code)]
    fn get_index(row: char, col: char) -> usize {
        let row = row as usize - '1' as usize;
        let col = col as usize - 'a' as usize;
        row * 8 + col
    }
}

#[wasm_bindgen]
impl ChessGame {
    #[must_use] pub fn new() -> ChessGame {
        set_panic_hook();
        let game = Game::new();
        let game_board = [Piece::Empty; 64];

        let mut chess_game = ChessGame { game, game_board };
        chess_game.update_game_board();
        console_log!("{}", chess_game.game);
        chess_game
    }

    pub fn play_move(
        &mut self,
        from1: char,
        from2: char,
        to1: char,
        to2: char,
    ) -> Option<UserOutputWrapper> {
        let user_output = self
            .game
            .process_input(&UserInput::Move((from1, from2).into(), (to1, to2).into()))
            .map(UserOutputWrapper);
        self.update_game_board();
        user_output
    }

    pub fn play_attacking_king(&mut self) -> Option<UserOutputWrapper> {
        let possible_moves = self.game.get_all_currently_valid_moves();
        if possible_moves.is_empty() {
            console_log!("Something went wrong. Function was probably called after check mate or stale mate.");
            return None;
        }

        let move_to_play = possible_moves
            .iter()
            .find(|mv| {
                let mut game = self.game.clone();
                match game.process_input(&UserInput::Move(mv.from, mv.to)) {
                    Some(UserOutput::CheckMate) => true,
                    _ => game.check(self.game.turn.invert()),
                }
            })
            .unwrap_or_else(|| {
                match possible_moves.iter().find(|mv| mv.captured_piece.is_some()) {
                    Some(mv) => mv,
                    None => {
                        let random_index =
                            (js_sys::Math::random() * (possible_moves.len() as f64 - 1.0)) as usize;
                        &possible_moves[random_index]
                    }
                }
            });

        let user_output = self
            .game
            .process_input(&UserInput::Move(move_to_play.from, move_to_play.to))
            .map(UserOutputWrapper);
        self.update_game_board();
        console_log!("{}", self.game);
        user_output
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
                let random_index =
                    (js_sys::Math::random() * (possible_moves.len() as f64 - 1.0)) as usize;
                &possible_moves[random_index]
            }
        };
        console_log!("{move_to_play}");

        let user_output = self
            .game
            .process_input(&UserInput::Move(move_to_play.from, move_to_play.to))
            .map(UserOutputWrapper);
        self.update_game_board();
        console_log!("{}", self.game);
        user_output
    }

    #[must_use] pub fn get_game_board(&self) -> *const Piece {
        self.game_board.as_ptr()
    }

    #[must_use] pub fn render(&self) -> String {
        self.game.to_string()
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}
