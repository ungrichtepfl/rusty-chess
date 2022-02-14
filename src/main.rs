use std::collections::HashMap;
use std::fmt;

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
    position: Position,
    value: f32,
}

#[derive(Debug, Clone)]
struct Board {
    pieces: HashMap<Color, Vec<Piece>>,
    captured: HashMap<Color, Vec<Piece>>,
}

impl Board {
    fn new() -> Board {
        // White pieces
        let pawn_w1 = Piece { name: Name::Pawn, color: Color::White, position: ('a', '2'), value: 1.0 };
        let pawn_w2 = Piece { position: ('b', '2'), ..pawn_w1.clone() };
        let pawn_w3 = Piece { position: ('b', '2'), ..pawn_w1.clone() };
        let pawn_w4 = Piece { position: ('c', '2'), ..pawn_w1.clone() };
        let pawn_w5 = Piece { position: ('d', '2'), ..pawn_w1.clone() };
        let pawn_w6 = Piece { position: ('e', '2'), ..pawn_w1.clone() };
        let pawn_w7 = Piece { position: ('f', '2'), ..pawn_w1.clone() };
        let pawn_w8 = Piece { position: ('g', '2'), ..pawn_w1.clone() };

        let knight_w1 = Piece { name: Name::Knight, color: Color::White, position: ('b', '1'), value: 3.0 };
        let knight_w2 = Piece { position: ('g', '1'), ..knight_w1.clone() };

        let bishop_w1 = Piece { name: Name::Bishop, color: Color::White, position: ('c', '1'), value: 3.0 };
        let bishop_w2 = Piece { position: ('f', '1'), ..bishop_w1.clone() };

        let rook_w1 = Piece { name: Name::Rook, color: Color::White, position: ('a', '1'), value: 5.0 };
        let rook_w2 = Piece { position: ('h', '1'), ..rook_w1.clone() };

        let queen_w = Piece { name: Name::Queen, color: Color::White, position: ('d', '1'), value: 8.0 };

        let king_w = Piece { name: Name::King, color: Color::White, position: ('e', '1'), value: 3.5 };

        // Black pieces
        let pawn_b1 = Piece { name: Name::Pawn, color: Color::Black, position: ('a', '2'), value: 1.0 };
        let pawn_b2 = Piece { position: ('b', '2'), ..pawn_b1.clone() };
        let pawn_b3 = Piece { position: ('b', '2'), ..pawn_b1.clone() };
        let pawn_b4 = Piece { position: ('c', '2'), ..pawn_b1.clone() };
        let pawn_b5 = Piece { position: ('d', '2'), ..pawn_b1.clone() };
        let pawn_b6 = Piece { position: ('e', '2'), ..pawn_b1.clone() };
        let pawn_b7 = Piece { position: ('f', '2'), ..pawn_b1.clone() };
        let pawn_b8 = Piece { position: ('g', '2'), ..pawn_b1.clone() };

        let knight_b1 = Piece { name: Name::Knight, color: Color::Black, position: ('b', '1'), value: 3.0 };
        let knight_b2 = Piece { position: ('g', '1'), ..knight_b1.clone() };

        let bishop_b1 = Piece { name: Name::Bishop, color: Color::Black, position: ('c', '1'), value: 3.0 };
        let bishop_b2 = Piece { position: ('f', '1'), ..bishop_b1.clone() };

        let rook_b1 = Piece { name: Name::Rook, color: Color::Black, position: ('a', '1'), value: 5.0 };
        let rook_b2 = Piece { position: ('h', '1'), ..rook_b1.clone() };

        let queen_b = Piece { name: Name::Queen, color: Color::Black, position: ('d', '1'), value: 8.0 };

        let king_b = Piece { name: Name::King, color: Color::Black, position: ('e', '1'), value: 3.5 };

        let pieces = HashMap::from(
            [
                (Color::White,
                 vec![
                     pawn_w1,
                     pawn_w2,
                     pawn_w3,
                     pawn_w4,
                     pawn_w5,
                     pawn_w6,
                     pawn_w7,
                     pawn_w8,
                     bishop_w1,
                     bishop_w2,
                     knight_w1,
                     knight_w2,
                     rook_w1,
                     rook_w2,
                     queen_w,
                     king_w,
                 ]),
                (Color::Black,
                 vec![
                     pawn_b1,
                     pawn_b2,
                     pawn_b3,
                     pawn_b4,
                     pawn_b5,
                     pawn_b6,
                     pawn_b7,
                     pawn_b8,
                     bishop_b1,
                     bishop_b2,
                     knight_b1,
                     knight_b2,
                     rook_b1,
                     rook_b2,
                     queen_b,
                     king_b,
                 ]),
            ]
        );
        let captured: HashMap<Color, Vec<Piece>> = HashMap::new();
        Board { pieces, captured }
    }
}


fn main() {
    let board = Board::new();
    println!("{:?}", board);
}
