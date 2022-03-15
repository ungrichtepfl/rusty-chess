use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter};

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
    history: Vec<(Piece, Move)>,
    able_to_long_castle: HashMap<Color, bool>,
    able_to_short_castle: HashMap<Color, bool>,
    protected_squares: HashMap<Color, Vec<Position>>,
    pieces_attacking_king: HashMap<Color, Vec<(Piece, Vec<Position>)>>,
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
        let history: Vec<(Piece, Move)> = Vec::new();
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

fn in_board_boundary(pos: Position, board: &Game) -> bool {
    match board.board.get(&pos) {
        // out of boundary
        None => { false }
        // in boundary
        _ => { true }
    }
}

fn no_obstacles_in_one_move(pos: Position, board: &Game, color: &Color, get_protected: bool) -> bool {
    match board.board.get(&pos) {
        // out of boundary
        None => { false }
        // no piece there: good
        Some(None) => { true }
        // cannot be same color
        Some(Some(p)) => { get_protected || p.color != *color }
    }
}

fn can_short_castle(board: &Game, color: &Color) -> bool {
    match color {
        Color::Black => {
            !check(board, color) && board.able_to_long_castle[color] && board.board[&('f', '8')].is_none() &&
                board.board[&('g', '8')].is_none() && !pos_protected(('f', '8'), board, color) &&
                !pos_protected(('g', '8'), board, color)
        }
        Color::White => {
            !check(board, color) && board.able_to_long_castle[color] && board.board[&('f', '1')].is_none() &&
                board.board[&('g', '1')].is_none() && !pos_protected(('f', '1'), board, color) &&
                !pos_protected(('g', '1'), board, color)
        }
    }
}

fn can_long_castle(board: &Game, color: &Color) -> bool {
    match color {
        Color::Black => {
            !check(board, color) && board.able_to_short_castle[&color] && board.board[&('b', '8')].is_none() &&
                board.board[&('c', '8')].is_none() && board.board[&('d', '8')].is_none() &&
                !pos_protected(('b', '8'), board, color) && !pos_protected(('c', '8'), board, color) &&
                !pos_protected(('d', '8'), board, color)
        }
        Color::White => {
            !check(board, color) && board.able_to_short_castle[&color] && board.board[&('b', '1')].is_none() &&
                board.board[&('c', '1')].is_none() && board.board[&('d', '1')].is_none() &&
                !pos_protected(('b', '1'), board, color) && !pos_protected(('c', '1'), board, color) &&
                !pos_protected(('d', '1'), board, color)
        }
    }
}

fn get_all_protected_squares(board: &Game) -> HashMap<Color, Vec<Position>> {
    let mut protected_squares_white: Vec<Position> = Vec::new();
    let mut protected_squares_black: Vec<Position> = Vec::new();
    for x in '1'..='8' {
        for y in 'a'..='h' {
            if board.board[&(x, y)].is_some() {
                let piece = board.board[&(x, y)].as_ref().unwrap();
                let possible_moves = possible_moves(&board, (x, y), true);
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

fn pos_protected(pos: Position, board: &Game, color: &Color) -> bool {
    let protected_positions: &Vec<Position> = if *color == Color::White {
        board.protected_squares[&Color::White].as_ref()
    } else {
        board.protected_squares[&Color::Black].as_ref()
    };
    for pos_attacked in protected_positions {
        if pos == *pos_attacked {
            return true;
        }
    }
    false
}

fn possible_horizontal_vertical_moves(board: &Game, pos: Position, piece: &Piece, get_protected: bool) -> Vec<Move> {
    let mut moves = Vec::new();

    // horizontal movement right
    let mut traversed_squares = vec![pos];
    for x in 1..=8 {
        let new_pos = padd(pos, (x, 0));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }

    // horizontal movement left
    traversed_squares = vec![pos];
    for x in (-8..=-1).rev() {
        let new_pos = padd(pos, (x, 0));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }

    // vertical movement up
    traversed_squares = vec![pos];
    for y in 1..=8 {
        let new_pos = padd(pos, (0, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }

    // vertical movement down
    traversed_squares = vec![pos];
    for y in (-8..=-1).rev() {
        let new_pos = padd(pos, (0, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }
    moves
}


fn possible_diagonal_moves(board: &Game, pos: Position, piece: &Piece, get_protected: bool) -> Vec<Move> {
    let mut moves = Vec::new();

    // right diagonal up
    let mut traversed_squares = vec![pos];
    for (x, y) in (1..=8).zip(1..=8) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }

    // right diagonal down
    traversed_squares = vec![pos];
    for (x, y) in (-8..=-1).rev().zip((-8..=-1).rev()) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }


    // left diagonal up
    traversed_squares = vec![pos];
    for (x, y) in (-8..=-1).rev().zip(1..=8) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }

    // left diagonal down
    traversed_squares = vec![pos];
    for (x, y) in (1..=8).zip((-8..=-1).rev()) {
        let new_pos = padd(pos, (x, y));
        if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
            break;
        }
        traversed_squares.push(new_pos);
        moves.push(
            Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: board.board[&new_pos].clone(),
            }
        )
    }
    moves
}


fn pieces_attacking_king(board: &Game) -> HashMap<Color, Vec<(Piece, Vec<Position>)>> {
    let mut pieces_white: Vec<(Piece, Vec<Position>)> = Vec::new();
    let mut pieces_black: Vec<(Piece, Vec<Position>)> = Vec::new();
    for x in 'a'..='h' {
        for y in '1'..='8' {
            let moves = possible_moves(board, (x, y.clone()), false);
            for mv in moves {
                if mv.captured_piece.is_some() {
                    if mv.captured_piece.unwrap().name == Name::King {
                        if mv.piece.color.invert() == Color::White {
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
            (Color::White, pieces_white),
            (Color::White, pieces_black)
        ]
    )
}

fn check(board: &Game, color: &Color) -> bool {
    !board.pieces_attacking_king[color].is_empty()
}

fn possible_moves(board: &Game, pos: Position, get_protected: bool) -> Vec<Move> {
    // todo what todo with get protected and check
    if board.board[&pos].is_none() {
        return Vec::new();
    }

    let piece = board.board[&pos].as_ref().unwrap();

    if check(board, &piece.color) && board.pieces_attacking_king[&piece.color].len() > 1 && piece.name != Name::King {
        // double check: only king can move
        return Vec::new();
    }

    let mut moves: Vec<Move> = Vec::new();


    match piece.name {
        Name::King => {
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    let new_pos = padd(pos, (x, y));
                    if no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) &&
                        !pos_protected(new_pos, &board, &piece.color) {
                        moves.push(
                            Move {
                                piece: piece.clone(),
                                move_type: MoveType::Normal,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: board.board[&new_pos].clone(),
                            }
                        )
                    }
                }
            }

            if can_long_castle(board, &piece.color) {
                moves.push(
                    Move {
                        piece: piece.clone(),
                        move_type: MoveType::LongCastle,
                        from: pos,
                        to: padd(pos, (-2, 0)),
                        traversed_squares: vec![pos, padd(pos, (-1, 0)), padd(pos, (-2, 0))],
                        captured_piece: board.board[&padd(pos, (-2, 0))].clone(),
                    }
                )
            }
            if can_short_castle(board, &piece.color) {
                moves.push(
                    Move {
                        piece: piece.clone(),
                        move_type: MoveType::ShortCastle,
                        from: pos,
                        to: padd(pos, (2, 0)),
                        traversed_squares: vec![pos, padd(pos, (1, 0)), padd(pos, (2, 0))],
                        captured_piece: board.board[&padd(pos, (2, 0))].clone(),
                    }
                )
            }
        }

        Name::Queen => {
            moves.append(possible_horizontal_vertical_moves(&board, pos, &piece, get_protected).as_mut());

            moves.append(possible_diagonal_moves(&board, pos, &piece, get_protected).as_mut());
        }

        Name::Rook => {
            moves.append(possible_horizontal_vertical_moves(&board, pos, &piece, get_protected).as_mut());
        }

        Name::Bishop => {
            moves.append(possible_diagonal_moves(&board, pos, &piece, get_protected).as_mut());
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
                if no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
                    moves.push(
                        Move {
                            piece: piece.clone(),
                            move_type: MoveType::Jump,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: board.board[&new_pos].clone(),
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
                if !no_obstacles_in_one_move(new_pos, &board, &piece.color, get_protected) {
                    break;
                }
                moves.push(
                    Move {
                        piece: piece.clone(),
                        move_type: MoveType::Normal,
                        from: pos,
                        to: new_pos,
                        traversed_squares: vec![pos, new_pos],
                        captured_piece: board.board[&new_pos].clone(),
                    }
                )
            }

            // check if able to capture a piece
            for new_pos_int in [(1, direction), (-1, direction)] {
                let new_pos = padd(pos, new_pos_int);
                match board.board.get(&new_pos) {
                    Some(Some(other_piece)) => {
                        if other_piece.color != piece.color {
                            moves.push(Move {
                                piece: piece.clone(),
                                move_type: MoveType::Normal,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: board.board[&new_pos].clone(),
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
                            piece: piece.clone(),
                            move_type: MoveType::Enpassant,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: board.board[&new_pos].clone(),
                        }
                    )
                }
            }
        }
    }


    if check(board, &piece.color) && piece.name != Name::King {
        // filter out moves that do not protect the king. Only for pieces other than the king.
        debug_assert_eq!(1, board.pieces_attacking_king[&piece.color].len(), "More than one piece attacking the king");
        let (_, pos) = board.pieces_attacking_king[&piece.color][0].clone();
        moves = moves.drain(..).filter(|x| pos.contains(&x.to)).collect();
    }

    moves
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

fn main() {
    let board = Game::new();
    println!("{}", board);
    println!("{:?}", possible_moves(&board, ('g', '1'), true));
    println!("{:?}", possible_moves(&board, ('g', '1'), false));
}
