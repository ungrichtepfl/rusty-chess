#[macro_use]
mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use rusty_chess_core::game::{Game, PieceType,Color, UserInput, UserOutput};

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
impl ChessGame {
    fn update_game_board(&mut self)  {
        for (i, piece) in self.game.get_board_array().iter().enumerate() {
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
    pub fn new() -> ChessGame {
        set_panic_hook();
        let game = Game::new();
        let game_board = [Piece::Empty; 64];

        let mut chess_game = ChessGame { game, game_board };
        chess_game.update_game_board();
        console_log!("{}",chess_game.game);
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
            .map(|x| UserOutputWrapper(x));
        self.update_game_board();
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

        let user_output = self
            .game
            .process_input(&UserInput::Move(move_to_play.from, move_to_play.to))
            .map(|x| UserOutputWrapper(x));
        self.update_game_board();
        console_log!("{}",self.game);
        user_output
    }

    pub fn get_game_board(&self) -> *const Piece {
        self.game_board.as_ptr()
    }

    pub fn render(&self) -> String {
        self.game.to_string()
    }
}
