use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
enum Name {
    Pawn,
    Bishop,
    Knight,
    King,
    Rook,
    Queen,
}

type Position = (char, char);


#[derive(Debug, Clone)]
struct Piece {
    name: Name,
    color: Color,
    value: f32,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.name {
            // unicode representation (https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode):
            Name::Pawn => if self.color == Color::White { write!(f, "\u{265F}") } else { write!(f, "\u{2659}") },
            Name::Bishop => if self.color == Color::White { write!(f, "\u{265D}") } else { write!(f, "\u{2657}") },
            Name::Knight => if self.color == Color::White { write!(f, "\u{265E}") } else { write!(f, "\u{2658}") },
            Name::King => if self.color == Color::White { write!(f, "\u{265A}") } else { write!(f, "\u{2654}") },
            Name::Rook => if self.color == Color::White { write!(f, "\u{265C}") } else { write!(f, "\u{2656}") },
            Name::Queen => if self.color == Color::White { write!(f, "\u{265B}") } else { write!(f, "\u{2655}") },
        }
    }
}

#[derive(Debug, Clone)]
enum MoveType {
    Vertical,
    Horizontal,
    Diagonal,
    Jump,
    Enpassant,
    LongCastle,
    ShortCastle,
}

#[derive(Debug, Clone)]
struct Move {
    from: Position,
    to: Position,
    move_type: MoveType,
}


#[derive(Debug, Clone)]
struct Board {
    pieces: HashMap<Position, Option<Piece>>,
    captured: HashMap<Color, Vec<Piece>>,
    history: Vec<(Piece, Move)>,
}


impl Piece {
    fn new(name: Name, color: Color) -> Piece {
        match name {
            Name::Pawn => Piece { name, color, value: 1.0 },
            Name::Knight => Piece { name, color, value: 3.0 },
            Name::Bishop => Piece { name, color, value: 3.0 },
            Name::Rook => Piece { name, color, value: 5.0 },
            Name::Queen => Piece { name, color, value: 8.0 },
            Name::King => Piece { name, color, value: 3.5 },
        }
    }
}

impl Board {
    fn new() -> Board {
        // White pieces
        let pawn_w = Piece::new(Name::Pawn, Color::White);

        let knight_w = Piece::new(Name::Knight, Color::White);

        let bishop_w = Piece::new(Name::Bishop, Color::White);

        let rook_w = Piece::new(Name::Rook, Color::White);

        let queen_w = Piece::new(Name::Queen, Color::White);

        let king_w = Piece::new(Name::King, Color::White);

        // Black pieces
        let pawn_b = Piece::new(Name::Pawn, Color::Black);

        let knight_b = Piece::new(Name::Knight, Color::Black);

        let bishop_b = Piece::new(Name::Bishop, Color::Black);

        let rook_b = Piece::new(Name::Rook, Color::Black);

        let queen_b = Piece::new(Name::Queen, Color::Black);

        let king_b = Piece::new(Name::King, Color::Black);

        let mut pieces: HashMap<Position, Option<Piece>> = HashMap::new();
        for x in 'a'..='h' {
            for y in '1'..='8' {
                if y == '2' {
                    pieces.insert((x.clone(), y), Some(pawn_w.clone()));
                } else if y == '7' {
                    pieces.insert((x.clone(), y), Some(pawn_b.clone()));
                } else if y == '1' && (x == 'a' || x == 'h') {
                    pieces.insert((x.clone(), y), Some(rook_w.clone()));
                } else if y == '8' && (x == 'a' || x == 'h') {
                    pieces.insert((x.clone(), y), Some(rook_b.clone()));
                } else if y == '1' && (x == 'b' || x == 'g') {
                    pieces.insert((x.clone(), y), Some(knight_w.clone()));
                } else if y == '8' && (x == 'b' || x == 'g') {
                    pieces.insert((x.clone(), y), Some(knight_b.clone()));
                } else if y == '1' && (x == 'c' || x == 'f') {
                    pieces.insert((x.clone(), y), Some(bishop_w.clone()));
                } else if y == '8' && (x == 'c' || x == 'f') {
                    pieces.insert((x.clone(), y), Some(bishop_b.clone()));
                } else if y == '1' && x == 'd' {
                    pieces.insert((x.clone(), y), Some(queen_w.clone()));
                } else if y == '8' && x == 'd' {
                    pieces.insert((x.clone(), y), Some(queen_b.clone()));
                } else if y == '1' && x == 'e' {
                    pieces.insert((x.clone(), y), Some(king_w.clone()));
                } else if y == '8' && x == 'e' {
                    pieces.insert((x.clone(), y), Some(king_b.clone()));
                } else {
                    pieces.insert((x.clone(), y), None);
                }
            }
        }
        let captured: HashMap<Color, Vec<Piece>> = HashMap::new();
        let history: Vec<(Piece, Move)> = Vec::new();
        Board { pieces, captured, history }
    }
}

fn padd(pos: Position, to_add: (i16, i16)) -> Position {
    let new_x = (pos.0 as i16 + to_add.0) as u8 as char;
    let new_y = (pos.1 as i16 + to_add.1) as u8 as char;

    debug_assert!('a' <= new_x && new_x <= 'h' && '1' <= new_y && new_y <= '8', "New position out of range.");

    (new_x, new_y)
}

fn p2num(pos: Position) -> (u8, u8) {
    (pos.0 as u8, pos.1 as u8)
}

fn valid_pos_in_one_move(pos: Position, board: &Board, color: &Color) -> bool {
    match board.pieces.get(&pos) {
        // out of boundary
        None => { false }
        // no piece there good
        Some(None) => { true }
        // cannot be same color
        Some(Some(p)) => { p.color != *color }
    }
}

fn can_long_castle(board: &Board, color: &Color) -> bool {
    unimplemented!()
}

fn can_short_castle(board: &Board, color: &Color) -> bool {
    unimplemented!()
}

fn possible_moves(board: &Board, pos: Position) -> Vec<Move> {
    if board.pieces[&pos].is_none() {
        return Vec::new();
    }

    let mut moves: Vec<Move> = Vec::new();

    let piece = board.pieces[&pos].as_ref().unwrap();
    match piece.name {
        Name::King => {
            if valid_pos_in_one_move(padd(pos, (1, 0)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Horizontal,
                        from: pos,
                        to: padd(pos, (1, 0)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (-1, 0)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Horizontal,
                        from: pos,
                        to: padd(pos, (-1, 0)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (0, 1)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Vertical,
                        from: pos,
                        to: padd(pos, (0, 1)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (0, -1)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Vertical,
                        from: pos,
                        to: padd(pos, (0, -1)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (1, 1)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Vertical,
                        from: pos,
                        to: padd(pos, (1, 1)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (-1, 1)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Vertical,
                        from: pos,
                        to: padd(pos, (-1, 1)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (1, -1)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Vertical,
                        from: pos,
                        to: padd(pos, (1, -1)),
                    });
            }
            if valid_pos_in_one_move(padd(pos, (-1, -1)), &board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::Vertical,
                        from: pos,
                        to: padd(pos, (-1, -1)),
                    });
            }

            if can_long_castle(board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::LongCastle,
                        from: pos,
                        to: padd(pos, (-2, 0)),
                    }
                )
            }
            if can_short_castle(board, &piece.color) {
                moves.push(
                    Move {
                        move_type: MoveType::LongCastle,
                        from: pos,
                        to: padd(pos, (2, 0)),
                    }
                )
            }
        }
        Name::Queen => {}
        Name::Rook => {}
        Name::Bishop => {}
        Name::Knight => {}
        Name::Pawn => {}
    }
    moves
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for _ in 1..=8 {
            res.push_str("\t-");
        }
        res.push_str("\n");

        for y in ('1'..='8').rev() {
            res.push_str(format!("{}|\t", y).as_str());
            for x in 'a'..='h' {
                match &self.pieces[&(x, y)] {
                    None => res.push_str("  |\t"),
                    Some(piece) => {
                        res.push_str(format!("{}|\t", piece).as_str())
                    }
                }
            }
            res.push_str(format!("\n").as_str());

            for _ in 1..=8 {
                res.push_str("\t-");
            }
            res.push_str("\n");
        }
        for x in 'a'..='h' {
            res.push_str(format!("\t{}", x).as_str());
        }
        res.push_str("\n");
        write!(f, "{}", res)
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);
}
