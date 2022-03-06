use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum MoveType {
    Straight,
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
    able_to_long_castle: HashMap<Color, bool>,
    able_to_short_caste: HashMap<Color, bool>,
    attacked_squares: HashMap<Color, Vec<Position>>,
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
        let able_to_castle = HashMap::from([(Color::White, true), (Color::Black, true)]);
        let mut attacked_squares_white: Vec<Position> = Vec::new();
        let mut attacked_squares_black: Vec<Position> = Vec::new();
        for x in 'a'..='h' {
            attacked_squares_white.push((x, '3'));
            attacked_squares_black.push((x, '6'));
        }
        let attacked_squares = HashMap::from([
            (Color::White, attacked_squares_white),
            (Color::Black, attacked_squares_black),
        ]);

        Board { pieces, captured, history, attacked_squares, able_to_long_castle: able_to_castle.clone(), able_to_short_caste: able_to_castle }
    }
}

fn padd(pos: Position, to_add: (i8, i8)) -> Position {
    let new_x = (pos.0 as i8 + to_add.0) as u8 as char;
    let new_y = (pos.1 as i8 + to_add.1) as u8 as char;

    (new_x, new_y)
}

fn p2num(pos: Position) -> (u8, u8) {
    (pos.0 as u8, pos.1 as u8)
}

fn no_obstacles_in_one_move(pos: Position, board: &Board, color: &Color) -> bool {
    match board.pieces.get(&pos) {
        // out of boundary
        None => { false }
        // no piece there: good
        Some(None) => { true }
        // cannot be same color
        Some(Some(p)) => { p.color != *color }
    }
}

fn can_short_castle(board: &Board, color: &Color) -> bool {
    match color {
        Color::Black => {
            board.able_to_long_castle[color] && board.pieces[&('f', '8')].is_none() && board.pieces[&('g', '8')].is_none()
        }
        Color::White => {
            board.able_to_long_castle[color] && board.pieces[&('f', '1')].is_none() && board.pieces[&('g', '1')].is_none()
        }
    }
}

fn can_long_castle(board: &Board, color: &Color) -> bool {
    match color {
        Color::Black => {
            board.able_to_short_caste[&color] && board.pieces[&('b', '8')].is_none() && board.pieces[&('c', '8')].is_none() && board.pieces[&('d', '8')].is_none()
        }
        Color::White => {
            board.able_to_short_caste[&color] && board.pieces[&('b', '1')].is_none() && board.pieces[&('c', '1')].is_none() && board.pieces[&('d', '1')].is_none()
        }
    }
}

fn get_attacked_squares(board: &Board) -> HashMap<Color, Vec<Position>> {
    let mut attacked_squares_white: Vec<Position> = Vec::new();
    let mut attacked_squares_black: Vec<Position> = Vec::new();
    for x in '1'..='8' {
        for y in 'a'..='h' {
            if board.pieces[&(x, y)].is_some() {
                let piece = board.pieces[&(x, y)].as_ref().unwrap();
                let posible_moves = possible_moves(&board, (x, y));
                for m in posible_moves {
                    if piece.color == Color::White {
                        attacked_squares_white.push(m.to);
                    } else {
                        attacked_squares_black.push(m.to);
                    }
                }
            }
        }
    }

    HashMap::from(
        [
            (Color::White, attacked_squares_white),
            (Color::Black, attacked_squares_black)
        ]
    )
}

fn pos_under_attack(pos: Position, board: &Board, piece: &Piece) -> bool {
    let attacked_positions: &Vec<Position> = if piece.color == Color::White {
        board.attacked_squares[&Color::White].as_ref()
    } else {
        board.attacked_squares[&Color::Black].as_ref()
    };
    for pos_attacked in attacked_positions {
        if pos == *pos_attacked {
            return true;
        }
    }
    false
}

fn possible_horizontal_vertical_moves(board: &Board, pos: Position, piece: &Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    // horizontal movement right
    for x in 1..=8 {
        let new_pos = padd(pos, (x, 0));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }

    // horizontal movement left
    for x in (-8..=-1).rev() {
        let new_pos = padd(pos, (x, 0));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }

    // vertical movement up
    for y in 1..=8 {
        let new_pos = padd(pos, (0, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }

    // vertical movement down
    for y in (-8..=-1).rev() {
        let new_pos = padd(pos, (0, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }
    moves
}


fn possible_diagonal_moves(board: &Board, pos: Position, piece: &Piece) -> Vec<Move> {
    let mut moves = Vec::new();

    // right diagonal up
    for (x, y) in (1..=8).zip(1..=8) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }

    // right diagonal down
    for (x, y) in (-8..=-1).rev().zip((-8..=-1).rev()) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }


    // left diagonal up
    for (x, y) in (-8..=-1).rev().zip(1..=8) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }

    // left diagonal down
    for (x, y) in (1..=8).zip((-8..=-1).rev()) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
            break;
        }
        moves.push(
            Move {
                move_type: MoveType::Straight,
                from: pos,
                to: new_pos,
            }
        )
    }
    moves
}


fn possible_moves(board: &Board, pos: Position) -> Vec<Move> {
    if board.pieces[&pos].is_none() {
        return Vec::new();
    }

    let mut moves: Vec<Move> = Vec::new();

    let piece = board.pieces[&pos].as_ref().unwrap();
    match piece.name {
        Name::King => {
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    let new_pos = padd(pos, (x, y));
                    if no_obstacles_in_one_move(new_pos, &board, &piece.color) {
                        moves.push(
                            Move {
                                move_type: MoveType::Straight,
                                from: pos,
                                to: new_pos,
                            }
                        )
                    }
                }
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
                        move_type: MoveType::ShortCastle,
                        from: pos,
                        to: padd(pos, (2, 0)),
                    }
                )
            }
        }

        Name::Queen => {
            moves.append(possible_horizontal_vertical_moves(&board, pos, &piece).as_mut());

            moves.append(possible_diagonal_moves(&board, pos, &piece).as_mut());
        }

        Name::Rook => {
            moves.append(possible_horizontal_vertical_moves(&board, pos, &piece).as_mut());
        }

        Name::Bishop => {
            moves.append(possible_diagonal_moves(&board, pos, &piece).as_mut());
        }

        Name::Knight => {
            let rel_pos_to_iter: [(i8, i8); 8] = [
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (-1, 2),
                (1, -2),
                (-1, -2),
            ];

            for pos_int in rel_pos_to_iter {
                let new_pos = padd(pos, pos_int);
                if no_obstacles_in_one_move(new_pos, &board, &piece.color) {
                    moves.push(
                        Move {
                            move_type: MoveType::Jump,
                            from: pos,
                            to: new_pos,
                        }
                    )
                }
            }
        }

        Name::Pawn => {
            let (direction, rel_pos_to_iter) =
                if piece.color == Color::White {
                    (1, if pos.1 == '2' { 1..3 } else { 1..2 })
                } else {
                    (-1, if pos.1 == '7' { 1..3 } else { 1..2 })
                };

            // Normal moves
            for y in rel_pos_to_iter {
                let new_pos = padd(pos, (0, direction * y));
                if !no_obstacles_in_one_move(new_pos, &board, &piece.color) {
                    break;
                }
                moves.push(
                    Move {
                        move_type: MoveType::Straight,
                        from: pos,
                        to: new_pos,
                    }
                )
            }

            // check if able to capture a piece
            for new_pos_int in [(1, direction), (-1, direction)] {
                let new_pos = padd(pos, new_pos_int);
                match board.pieces.get(&new_pos) {
                    Some(Some(other_piece)) => {
                        if other_piece.color != piece.color {
                            moves.push(Move {
                                move_type: MoveType::Straight,
                                from: pos,
                                to: new_pos,
                            })
                        }
                    }
                    _ => {} // do nothing
                }
            }

            // Enpassant
            if !board.history.is_empty() {
                let (last_move_piece, last_move) = board.history.last().unwrap();

                if last_move_piece.name == Name::Pawn &&
                    (last_move.from.1 as i8 - last_move.to.1 as i8).abs() == 2 &&
                    (last_move.to == padd(pos, (1, 0)) || last_move.to == padd(pos, (-1, 0))) {
                    let new_pos = if last_move.to == padd(pos, (1, 0)) {
                        padd(pos, (1, direction))
                    } else {
                        padd(pos, (-1, direction))
                    };
                    moves.push(
                        Move {
                            move_type: MoveType::Enpassant,
                            from: pos,
                            to: new_pos,
                        }
                    )
                }
            }
        }
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
    println!("{:?}", possible_moves(&board, ('g', '1')));
}
