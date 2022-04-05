use std::collections::HashMap;
use std::{fmt, io};
use std::fmt::{Formatter};
use std::io::BufRead;
use std::process::exit;

use regex::Regex;

use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Color {
    White,
    Black,
}

impl Color {
    fn invert(&self) -> Color {
        match self {
            Color::White => { Color::Black }
            Color::Black => { Color::White }
        }
    }
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
    value: i8,
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
    Normal,
    Jump,
    Enpassant,
    LongCastle,
    ShortCastle,
}

#[derive(Debug, Clone)]
struct Move {
    piece: Piece,
    from: Position,
    to: Position,
    move_type: MoveType,
    traversed_squares: Vec<Position>,
    captured_piece: Option<Piece>,
}

#[derive(Debug, Clone)]
struct Game {
    turn: Color,
    board: HashMap<Position, Option<Piece>>,
    captured: HashMap<Color, Vec<Piece>>,
    history: Vec<Move>,
    able_to_long_castle: HashMap<Color, bool>,
    able_to_short_castle: HashMap<Color, bool>,
    protected_squares: HashMap<Color, Vec<Position>>,
    pieces_attacking_king: HashMap<Color, Vec<(Piece, Vec<Position>)>>,
}


impl Piece {
    fn new(name: Name, color: Color) -> Piece {
        match name {
            Name::Pawn => Piece { name, color, value: 1 },
            Name::Knight => Piece { name, color, value: 3 },
            Name::Bishop => Piece { name, color, value: 3 },
            Name::Rook => Piece { name, color, value: 5 },
            Name::Queen => Piece { name, color, value: 8 },
            Name::King => Piece { name, color, value: 0 },
        }
    }
}

impl Game {
    fn new() -> Game {
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

        let mut board: HashMap<Position, Option<Piece>> = HashMap::new();
        for x in 'a'..='h' {
            for y in '1'..='8' {
                if y == '2' {
                    board.insert((x.clone(), y), Some(pawn_w.clone()));
                } else if y == '7' {
                    board.insert((x.clone(), y), Some(pawn_b.clone()));
                } else if y == '1' && (x == 'a' || x == 'h') {
                    board.insert((x.clone(), y), Some(rook_w.clone()));
                } else if y == '8' && (x == 'a' || x == 'h') {
                    board.insert((x.clone(), y), Some(rook_b.clone()));
                } else if y == '1' && (x == 'b' || x == 'g') {
                    board.insert((x.clone(), y), Some(knight_w.clone()));
                } else if y == '8' && (x == 'b' || x == 'g') {
                    board.insert((x.clone(), y), Some(knight_b.clone()));
                } else if y == '1' && (x == 'c' || x == 'f') {
                    board.insert((x.clone(), y), Some(bishop_w.clone()));
                } else if y == '8' && (x == 'c' || x == 'f') {
                    board.insert((x.clone(), y), Some(bishop_b.clone()));
                } else if y == '1' && x == 'd' {
                    board.insert((x.clone(), y), Some(queen_w.clone()));
                } else if y == '8' && x == 'd' {
                    board.insert((x.clone(), y), Some(queen_b.clone()));
                } else if y == '1' && x == 'e' {
                    board.insert((x.clone(), y), Some(king_w.clone()));
                } else if y == '8' && x == 'e' {
                    board.insert((x.clone(), y), Some(king_b.clone()));
                } else {
                    board.insert((x.clone(), y), None);
                }
            }
        }
        let captured: HashMap<Color, Vec<Piece>> = HashMap::new();
        let history: Vec<Move> = Vec::new();
        let able_to_castle = HashMap::from([(Color::White, true), (Color::Black, true)]);
        let mut protected_squares_white: Vec<Position> = Vec::new();
        let mut protected_squares_black: Vec<Position> = Vec::new();
        for x in 'a'..='h' {
            protected_squares_white.push((x, '3'));
            protected_squares_white.push((x, '2'));
            protected_squares_black.push((x, '6'));
            protected_squares_black.push((x, '7'));
            if x != 'a' || x != 'h' {
                protected_squares_white.push((x, '1'));
                protected_squares_black.push((x, '8'));
            }
        }

        let protected_squares = HashMap::from([
            (Color::White, protected_squares_white),
            (Color::Black, protected_squares_black),
        ]);

        let pieces_attacking_king = HashMap::from(
            [
                (Color::White, Vec::new()),
                (Color::Black, Vec::new())
            ]
        );

        Game {
            turn: Color::White,
            board,
            captured,
            history,
            protected_squares,
            pieces_attacking_king,
            able_to_long_castle: able_to_castle.clone(),
            able_to_short_castle: able_to_castle,
        }
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

fn in_board_boundary(pos: Position, game: &Game) -> bool {
    match game.board.get(&pos) {
        // out of boundary
        None => { false }
        // in boundary
        _ => { true }
    }
}

enum Obstacle {
    OutOfBoundary,
    Piece(Color),
}

fn obstacles_in_one_move(pos: Position, game: &Game, color: &Color) -> Option<Obstacle> {
    match game.board.get(&pos) {
        // out of boundary
        None => { Some(Obstacle::OutOfBoundary) }
        // no piece there: good
        Some(None) => { None }
        // cannot be same color and if you want to get protected any piece is an obstacle
        Some(Some(p)) => { Some(Obstacle::Piece(p.color.clone())) }
    }
}


fn can_short_castle(game: &Game, color: &Color) -> bool {
    match color {
        Color::Black => {
            !check(game, color) && game.able_to_short_castle[color] && game.board[&('f', '8')].is_none() &&
                game.board[&('g', '8')].is_none() && !pos_protected(('f', '8'), game, &color.invert()) &&
                !pos_protected(('g', '8'), game, &color.invert())
        }
        Color::White => {
            !check(game, color) && game.able_to_short_castle[color] && game.board[&('f', '1')].is_none() &&
                game.board[&('g', '1')].is_none() && !pos_protected(('f', '1'), game, &color.invert()) &&
                !pos_protected(('g', '1'), game, &color.invert())
        }
    }
}

fn can_long_castle(game: &Game, color: &Color) -> bool {
    match color {
        Color::Black => {
            !check(game, color) && game.able_to_long_castle[&color] && game.board[&('b', '8')].is_none() &&
                game.board[&('c', '8')].is_none() && game.board[&('d', '8')].is_none() &&
                !pos_protected(('c', '8'), game, &color.invert()) &&
                !pos_protected(('d', '8'), game, &color.invert())
        }
        Color::White => {
            !check(game, color) && game.able_to_long_castle[&color] && game.board[&('b', '1')].is_none() &&
                game.board[&('c', '1')].is_none() && game.board[&('d', '1')].is_none() &&
                !pos_protected(('c', '1'), game, &color.invert()) &&
                !pos_protected(('d', '1'), game, &color.invert())
        }
    }
}

fn get_all_protected_squares(game: &Game, filter_pinned: bool) -> HashMap<Color, Vec<Position>> {
    let mut protected_squares_white: Vec<Position> = Vec::new();
    let mut protected_squares_black: Vec<Position> = Vec::new();
    for x in 'a'..='h' {
        for y in '1'..='8' {
            if game.board[&(x, y)].is_some() {
                let piece = game.board[&(x, y)].as_ref().unwrap();
                let possible_moves = possible_moves(&game, (x, y), true, filter_pinned);
                for m in possible_moves {
                    if piece.color == Color::White {
                        protected_squares_white.push(m.to);
                    } else {
                        protected_squares_black.push(m.to);
                    }
                }
            }
        }
    }

    HashMap::from(
        [
            (Color::White, protected_squares_white),
            (Color::Black, protected_squares_black)
        ]
    )
}

fn pos_protected(pos: Position, game: &Game, color: &Color) -> bool {
    let protected_positions: &Vec<Position> = if *color == Color::White {
        game.protected_squares[&Color::White].as_ref()
    } else {
        game.protected_squares[&Color::Black].as_ref()
    };
    for pos_protected in protected_positions {
        if pos == *pos_protected {
            return true;
        }
    }
    false
}

fn get_moves_in_one_direction<I1, I2>(x_path: I1, y_path: I2, game: &Game, pos: Position, piece: &Piece, get_protected: bool) -> Vec<Move>
    where I1: Iterator<Item=i8>, I2: Iterator<Item=i8> {
    let mut moves = Vec::new();

    let mut traversed_squares = vec![pos];
    for (x, y) in x_path.zip(y_path) {
        let new_pos = padd(pos, (x, y));
        let obstacle = obstacles_in_one_move(new_pos, &game, &piece.color);

        if obstacle.is_some() {
            match obstacle.unwrap() {
                Obstacle::OutOfBoundary => { break; }
                Obstacle::Piece(obstacle_color) => {
                    if !get_protected && obstacle_color == piece.color {
                        break;
                    } else {
                        traversed_squares.push(new_pos);
                        moves.push(
                            Move {
                                piece: piece.clone(),
                                move_type: MoveType::Normal,
                                from: pos,
                                to: new_pos,
                                traversed_squares: traversed_squares.clone(),
                                captured_piece: game.board[&new_pos].clone(),
                            }
                        );
                        break;
                    }
                }
            }
        }

        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: game.board[&new_pos].clone(),
            }
        );
    }
    moves
}

fn possible_horizontal_vertical_moves(game: &Game, pos: Position, piece: &Piece, get_protected: bool) -> Vec<Move> {
    let mut moves = Vec::new();

    // horizontal movement right
    moves.append(get_moves_in_one_direction(1..=8, [0i8; 8].into_iter(), game, pos, piece, get_protected).as_mut());

    // horizontal movement left
    moves.append(get_moves_in_one_direction((-8..=-1).rev(), [0i8; 8].into_iter(), game, pos, piece, get_protected).as_mut());

    // vertical movement up
    moves.append(get_moves_in_one_direction([0i8; 8].into_iter(), 1..=8, game, pos, piece, get_protected).as_mut());

    // vertical movement down
    moves.append(get_moves_in_one_direction([0i8; 8].into_iter(), (-8..=-1).rev(), game, pos, piece, get_protected).as_mut());

    moves
}


fn possible_diagonal_moves(game: &Game, pos: Position, piece: &Piece, get_protected: bool) -> Vec<Move> {
    let mut moves = Vec::new();

    // right diagonal up
    moves.append(get_moves_in_one_direction(1..=8, 1..=8, game, pos, piece, get_protected).as_mut());

    // right diagonal down
    moves.append(get_moves_in_one_direction(1..=8, (-8..=-1).rev(), game, pos, piece, get_protected).as_mut());

    // left diagonal up
    moves.append(get_moves_in_one_direction((-8..=-1).rev(), 1..=8, game, pos, piece, get_protected).as_mut());

    // left diagonal down
    moves.append(get_moves_in_one_direction((-8..=-1).rev(), (-8..=-1).rev(), game, pos, piece, get_protected).as_mut());

    moves
}


fn pieces_attacking_king(game: &Game, filter_pinned: bool) -> HashMap<Color, Vec<(Piece, Vec<Position>)>> {
    let mut pieces_white: Vec<(Piece, Vec<Position>)> = Vec::new();
    let mut pieces_black: Vec<(Piece, Vec<Position>)> = Vec::new();
    for x in 'a'..='h' {
        for y in '1'..='8' {
            let moves = possible_moves(game, (x, y.clone()), false, filter_pinned);
            for mv in moves {
                if mv.captured_piece.is_some() {
                    if mv.captured_piece.unwrap().name == Name::King {
                        if mv.piece.color == Color::White {
                            pieces_white.push((mv.piece, mv.traversed_squares));
                        } else {
                            pieces_black.push((mv.piece, mv.traversed_squares));
                        }
                    }
                }
            }
        }
    }

    HashMap::from(
        [
            (Color::White, pieces_black),
            (Color::Black, pieces_white)
        ]
    )
}

fn no_possible_moves(game: &Game, color: &Color) -> bool {
    for x in 'a'..='h' {
        for y in '1'..='8' {
            let moves = possible_moves(game, (x, y.clone()), false, false);
            if !moves.is_empty() {
                if moves[0].piece.color == *color {
                    return false;
                }
            }
        }
    }
    true
}

#[inline]
fn check(game: &Game, color: &Color) -> bool {
    !game.pieces_attacking_king[color].is_empty()
}

fn possible_moves(game: &Game, pos: Position, get_protected: bool, filter_pinned: bool) -> Vec<Move> {
    if game.board[&pos].is_none() {
        return Vec::new();
    }

    let piece = game.board[&pos].as_ref().unwrap();

    if !get_protected && check(game, &piece.color) && game.pieces_attacking_king[&piece.color].len() > 1 && piece.name != Name::King {
        // double check: only king can move
        return Vec::new();
    }

    let mut moves: Vec<Move> = Vec::new();

    match piece.name {
        Name::King => {
            for (x, y) in [(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1, ), (0, -1), (-1, -1), (-1, 0)] {
                let new_pos = padd(pos, (x, y));
                match obstacles_in_one_move(new_pos, &game, &piece.color) {
                    None => {
                        if get_protected || !pos_protected(new_pos, game, &piece.color.invert()) {
                            moves.push(
                                Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: game.board[&new_pos].clone(),
                                }
                            );
                        }
                    }
                    Some(Obstacle::Piece(obstacle_color)) => {
                        if get_protected || (!pos_protected(new_pos, game, &piece.color.invert()) && piece.color != obstacle_color) {
                            moves.push(
                                Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: game.board[&new_pos].clone(),
                                }
                            );
                        }
                    }
                    Some(Obstacle::OutOfBoundary) => {
                        // nothing to do
                    }
                }
            }

            if !get_protected && can_long_castle(game, &piece.color) {
                moves.push(
                    Move {
                        piece: piece.clone(),
                        move_type: MoveType::LongCastle,
                        from: pos,
                        to: padd(pos, (-2, 0)),
                        traversed_squares: vec![pos, padd(pos, (-1, 0)), padd(pos, (-2, 0))],
                        captured_piece: game.board[&padd(pos, (-2, 0))].clone(),
                    }
                )
            }
            if !get_protected && can_short_castle(game, &piece.color) {
                moves.push(
                    Move {
                        piece: piece.clone(),
                        move_type: MoveType::ShortCastle,
                        from: pos,
                        to: padd(pos, (2, 0)),
                        traversed_squares: vec![pos, padd(pos, (1, 0)), padd(pos, (2, 0))],
                        captured_piece: game.board[&padd(pos, (2, 0))].clone(),
                    }
                )
            }
        }

        Name::Queen => {
            moves.append(possible_horizontal_vertical_moves(&game, pos, &piece, get_protected).as_mut());

            moves.append(possible_diagonal_moves(&game, pos, &piece, get_protected).as_mut());
        }

        Name::Rook => {
            moves.append(possible_horizontal_vertical_moves(&game, pos, &piece, get_protected).as_mut());
        }

        Name::Bishop => {
            moves.append(possible_diagonal_moves(&game, pos, &piece, get_protected).as_mut());
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
                match obstacles_in_one_move(new_pos, &game, &piece.color) {
                    None => {
                        moves.push(
                            Move {
                                piece: piece.clone(),
                                move_type: MoveType::Jump,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: game.board[&new_pos].clone(),
                            }
                        );
                    }
                    Some(Obstacle::Piece(obstacle_color)) => {
                        if get_protected || piece.color != obstacle_color {
                            moves.push(
                                Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Jump,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: game.board[&new_pos].clone(),
                                }
                            );
                        }
                    }
                    Some(Obstacle::OutOfBoundary) => {
                        // nothing to do
                    }
                }
            }
        }

        Name::Pawn => {  // fixme pawn move not possible h6h5
            let (direction, rel_pos_to_iter) =
                if piece.color == Color::White {
                    (1, if pos.1 == '2' { 1..3 } else { 1..2 })
                } else {
                    (-1, if pos.1 == '7' { 1..3 } else { 1..2 })
                };

            // Normal moves
            if !get_protected {
                for y in rel_pos_to_iter {
                    let new_pos = padd(pos, (0, direction * y));
                    match game.board.get(&new_pos) {
                        Some(None) => {
                            moves.push(
                                Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: game.board[&new_pos].clone(),
                                }
                            );
                        }
                        _ => { break; }
                    }
                }
            }

            // check if able to capture a piece
            for new_pos_int in [(1, direction), (-1, direction)] {
                let new_pos = padd(pos, new_pos_int);
                match game.board.get(&new_pos) {
                    Some(Some(other_piece)) => {
                        if get_protected || other_piece.color != piece.color {
                            moves.push(Move {
                                piece: piece.clone(),
                                move_type: MoveType::Normal,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: game.board[&new_pos].clone(),
                            });
                        }
                    }
                    Some(None) => {
                        if get_protected {
                            moves.push(Move {
                                piece: piece.clone(),
                                move_type: MoveType::Normal,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: game.board[&new_pos].clone(),
                            });
                        }
                    }
                    None => {}  // do nothing
                }
            }

            // Enpassant
            if !get_protected {
                if !game.history.is_empty() {
                    let last_move = game.history.last().unwrap();

                    if last_move.piece.name == Name::Pawn &&
                        (last_move.from.1 as i8 - last_move.to.1 as i8).abs() == 2 &&
                        (last_move.to == padd(pos, (1, 0)) || last_move.to == padd(pos, (-1, 0))) {
                        let new_pos = if last_move.to == padd(pos, (1, 0)) {
                            padd(pos, (1, direction))
                        } else {
                            padd(pos, (-1, direction))
                        };
                        moves.push(
                            Move {
                                piece: piece.clone(),
                                move_type: MoveType::Enpassant,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: game.board[&padd(new_pos, (0, -direction))].clone(),
                            }
                        );
                    }
                }
            }
        }
    }


    if !get_protected && check(game, &piece.color) && piece.name != Name::King {
        // filter out moves that do not protect the king. Only for pieces other than the king.
        debug_assert_eq!(1, game.pieces_attacking_king[&piece.color].len(), "More than one piece attacking the king");
        let (_, pos) = game.pieces_attacking_king[&piece.color][0].clone();
        moves = moves.drain(..).filter(|x| pos.contains(&x.to)).collect();
    }

    if filter_pinned {
        moves = moves.drain(..).filter(|x| piece_is_not_pinned(game, x)).collect()
    }

    moves
}

fn piece_is_not_pinned(game: &Game, mv: &Move) -> bool {
    if mv.piece.name == Name::King {
        true
    } else {
        let mut game_after_move = game.clone();
        game_after_move.turn = game_after_move.turn.invert();
        // update position
        game_after_move.board.insert(mv.from, None);
        game_after_move.board.insert(mv.to, Some(mv.piece.clone()));
        if mv.move_type == MoveType::Enpassant {
            let direction = if mv.piece.color == Color::White {
                1
            } else {
                -1
            };
            game_after_move.board.insert(padd(mv.to, (0, -direction)), None);
        }
        game_after_move.protected_squares = get_all_protected_squares(&game_after_move, false);
        game_after_move.pieces_attacking_king = pieces_attacking_king(&game_after_move, false);
        game_after_move.pieces_attacking_king[&mv.piece.color].is_empty()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?} -> {:?}", self.piece.name, self.from, self.to)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for _ in 1..=8 {
            res.push_str("\t-");
        }
        res.push_str("\n");

        for y in ('1'..='8').rev() {
            res.push_str(format!("{}|\t", y).as_str());
            for x in 'a'..='h' {
                match &self.board[&(x, y)] {
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

impl Move {
    fn construct_if_valid(game: &Game, from: Position, to: Position) -> Option<Move> {
        match game.board.get(&from) {
            Some(Some(piece)) => {
                if game.turn == piece.color {
                    let matching_moves: Vec<Move> = possible_moves(game, from, false, true).drain(..).filter(|x| x.to == to).collect();
                    if matching_moves.is_empty() {
                        None
                    } else {
                        debug_assert_eq!(1, matching_moves.len());
                        Some(matching_moves[0].clone())
                    }
                } else {
                    None
                }
            }
            _ => None
        }
    }
}

enum UserInput {
    Move(Position, Position),
    Promotion(Name),
    Draw,
    Resign,
}

enum UserOutput {
    CheckMate,
    StaleMate,
    InvalidMove,
    Promotion(Position),
    Draw,
}

fn move_piece(game: &mut Game, from: Position, to: Position) -> Option<UserOutput> {
    // todo pawns to other pieces
    // todo all the draws
    match Move::construct_if_valid(game, from, to) {
        Some(mv) => {
            game.turn = game.turn.invert();
            // update position
            game.board.insert(from, None);
            game.board.insert(to, Some(mv.piece.clone()));
            if mv.move_type == MoveType::Enpassant {
                let direction = if mv.piece.color == Color::White {
                    1
                } else {
                    -1
                };
                game.board.insert(padd(mv.to, (0, -direction)), None);
            }
            if mv.move_type == MoveType::LongCastle {
                if mv.piece.color == Color::White {
                    game.board.insert(('a', '1'), None);
                    game.board.insert(('d', '1'), Some(Piece::new(Name::Rook, Color::White)));
                } else {
                    game.board.insert(('a', '8'), None);
                    game.board.insert(('d', '8'), Some(Piece::new(Name::Rook, Color::Black)));
                }
            }
            if mv.move_type == MoveType::ShortCastle {
                if mv.piece.color == Color::White {
                    game.board.insert(('h', '1'), None);
                    game.board.insert(('f', '1'), Some(Piece::new(Name::Rook, Color::White)));
                } else {
                    game.board.insert(('h', '8'), None);
                    game.board.insert(('f', '8'), Some(Piece::new(Name::Rook, Color::Black)));
                }
            }
            // fixme circular relationship in those function. Dirty fix was used by checking bool get_protected when checking if check
            //  get_all_protected_squares has to be run before pieces_attacking_king right now
            game.protected_squares = get_all_protected_squares(game, true);
            game.pieces_attacking_king = pieces_attacking_king(game, true);

            if mv.captured_piece.is_some() {
                game.captured.entry(
                    mv.piece.color.clone()).or_insert(
                    Vec::new()).push(mv.captured_piece.clone().unwrap());
            }
            if (mv.piece.name == Name::King || mv.piece.name == Name::Rook)
                && (game.able_to_long_castle[&mv.piece.color] || game.able_to_short_castle[&mv.piece.color]) {
                if mv.piece.name == Name::King {
                    game.able_to_short_castle.insert(mv.piece.color.clone(), false);
                    game.able_to_long_castle.insert(mv.piece.color.clone(), false);
                } else {
                    let long_caste_pos: Position = if mv.piece.color == Color::White {
                        ('a', '1')
                    } else {
                        ('a', '8')
                    };
                    let short_caste_pos: Position = if mv.piece.color == Color::White {
                        ('h', '1')
                    } else {
                        ('h', '8')
                    };
                    if mv.from == long_caste_pos {
                        game.able_to_long_castle.insert(mv.piece.color.clone(), false);
                    } else if mv.from == short_caste_pos {
                        game.able_to_short_castle.insert(mv.piece.color.clone(), false);
                    }
                }
            }
            game.history.push(mv);
            if no_possible_moves(game, &game.turn) {
                return if check(game, &game.turn) {
                    Some(UserOutput::CheckMate)
                } else {
                    Some(UserOutput::StaleMate)
                };
            }

            None
        }
        None => Some(UserOutput::InvalidMove)
    }
}


fn parse_input_move(std_input: &String) -> Result<UserInput, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\s*)?([a-zA-Z])(?:\s*)?(\d)(?:\s*)?(?:-|->)?(?:\s*)?([a-zA-Z])(?:\s*)?(\d)(?:\s*)?").unwrap();
    }
    match RE.captures(std_input) {
        None => {
            if std_input.contains("Resign") || std_input.contains("resign") {
                Ok(UserInput::Resign)
            } else if std_input.contains("Draw") || std_input.contains("draw") {
                Ok(UserInput::Draw)
            } else {
                Err(String::from("Wrong input format, please input a move."))
            }
        }
        Some(cap) => {
            let from: Position = (cap[1].chars().nth(0).unwrap(), cap[2].chars().nth(0).unwrap());
            let to: Position = (cap[3].chars().nth(0).unwrap(), cap[4].chars().nth(0).unwrap());
            Ok(UserInput::Move(from, to))
        }
    }
}

fn headless_chess() {
    println!("Hello to rusty chess. Let's start a game:\n");
    let mut game = Game::new();
    let stdin = io::stdin();
    loop {
        println!("{:?}s turn. Please input a move (e.g. \"e2 e4\" moves piece on e2 to e4)", game.turn);
        println!("{}", game);
        let input_move = stdin.lock().lines().next().unwrap().unwrap();
        match parse_input_move(&input_move) {
            Err(e) => println!("{}", e),
            Ok(UserInput::Move(from, to)) => {
                let user_output = move_piece(&mut game, from, to);
                if user_output.is_some() {
                    match user_output.unwrap() {
                        UserOutput::InvalidMove => {
                            println!("Not a valid move please repeat a move.")
                        }
                        UserOutput::Draw => {
                            println!("It is a draw!");
                            println!("{}", game);
                            exit(0)
                        }
                        UserOutput::CheckMate => {
                            println!("{:?} has won!", game.turn.invert());
                            println!("{}", game);
                            exit(0)
                        }
                        UserOutput::StaleMate => {
                            println!("It is a draw stalemate!");
                            println!("{}", game);
                            exit(0)
                        }
                        UserOutput::Promotion(pos) => {
                            unreachable!();
                            //todo
                        }
                    }
                }
            }
            Ok(UserInput::Resign) => {
                println!("{:?} resigns!", game.turn);
                exit(0)
            }
            Ok(UserInput::Draw) => {
                println!("{:?} offers a draw does {:?} accept it? [y/N]", game.turn, game.turn.invert());
                let input_move = stdin.lock().lines().next().unwrap().unwrap();
                if input_move.contains("y") {
                    println!("It is a draw!");
                    exit(0)
                } else {
                    println!("Draw has been refused!")
                }
            }
            Ok(UserInput::Promotion(name)) => {
                println!("Promotion has been selected.")
                // todo
            }
        }
    }
}

fn main() {
    headless_chess();
}
