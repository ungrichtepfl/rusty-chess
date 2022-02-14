use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, write};

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

impl fmt::Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        match self {
            Name::Pawn => write!(f, "P"),
            Name::Bishop => write!(f, "B"),
            Name::Knight => write!(f, "H"),
            Name::King => write!(f, "K"),
            Name::Rook => write!(f, "R"),
            Name::Queen => write!(f, "Q")
        }
    }
}

type Position = (char, char);

#[derive(Debug, Clone)]
struct Piece {
    name: Name,
    color: Color,
    value: f32,
}

#[derive(Debug, Clone)]
struct Board {
    pieces: HashMap<Position, Option<Piece>>,
    captured: HashMap<Color, Vec<Piece>>,
}

impl Board {
    fn new() -> Board {
        // White pieces
        let pawn_w = Piece { name: Name::Pawn, color: Color::White, value: 1.0 };

        let knight_w = Piece { name: Name::Knight, color: Color::White, value: 3.0 };

        let bishop_w = Piece { name: Name::Bishop, color: Color::White, value: 3.0 };

        let rook_w = Piece { name: Name::Rook, color: Color::White, value: 5.0 };

        let queen_w = Piece { name: Name::Queen, color: Color::White, value: 8.0 };

        let king_w = Piece { name: Name::King, color: Color::White, value: 3.5 };

        // Black pieces
        let pawn_b = Piece { name: Name::Pawn, color: Color::Black, value: 1.0 };

        let knight_b = Piece { name: Name::Knight, color: Color::Black, value: 3.0 };

        let bishop_b = Piece { name: Name::Bishop, color: Color::Black, value: 3.0 };

        let rook_b = Piece { name: Name::Rook, color: Color::Black, value: 5.0 };

        let queen_b = Piece { name: Name::Queen, color: Color::Black, value: 8.0 };

        let king_b = Piece { name: Name::King, color: Color::Black, value: 3.5 };

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
        Board { pieces, captured }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for _ in 1..=8 {
            res.push_str("\t-");
        }
        res.push_str("\n");

        for y in ('1'..='8').rev() {
            res.push_str(format!("{}|\t",y).as_str());
            for x in 'a'..='h' {
                match &self.pieces[&(x, y)] {
                    None => res.push_str("  |\t"),
                    Some(piece) => {
                        let pre_fix = if piece.color == Color::White {
                            String::from("W")
                        } else {
                            String::from("B")
                        };
                        res.push_str(format!("{}{}|\t", pre_fix, piece.name).as_str())
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
            res.push_str(format!("\t{}",x).as_str());
        }
        res.push_str("\n");
        write!(f, "{}", res)
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);
}
