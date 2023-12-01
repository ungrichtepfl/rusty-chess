// standard imports
use std::collections::HashMap;
use std::fmt::{self, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn invert(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    King,
    Rook,
    Queen,
}

impl PieceType {
    fn value(&self) -> u8 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 8,
            PieceType::King => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub char, pub char);

impl Position {
    /// Adds a tuple of i8 to the position and returns a new position.
    /// No boundary check is done!
    fn add(&self, to_add: (i8, i8)) -> Position {
        let new_x = (self.0 as i8 + to_add.0) as u8 as char;
        let new_y = (self.1 as i8 + to_add.1) as u8 as char;

        Position(new_x, new_y)
    }
}

impl From<(char, char)> for Position {
    fn from(val: (char, char)) -> Self {
        Position(val.0, val.1)
    }
}
impl From<(&char, &char)> for Position {
    fn from(val: (&char, &char)) -> Self {
        Position(*val.0, *val.1)
    }
}
type BoardArray = [Option<Piece>; 64];

enum Obstacle {
    OutOfBoundary,
    Piece(Color),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.piece_type {
            // unicode representation (https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode):
            PieceType::Pawn => {
                if self.color == Color::White {
                    write!(f, "\u{265F}")
                } else {
                    write!(f, "\u{2659}")
                }
            }
            PieceType::Bishop => {
                if self.color == Color::White {
                    write!(f, "\u{265D}")
                } else {
                    write!(f, "\u{2657}")
                }
            }
            PieceType::Knight => {
                if self.color == Color::White {
                    write!(f, "\u{265E}")
                } else {
                    write!(f, "\u{2658}")
                }
            }
            PieceType::King => {
                if self.color == Color::White {
                    write!(f, "\u{265A}")
                } else {
                    write!(f, "\u{2654}")
                }
            }
            PieceType::Rook => {
                if self.color == Color::White {
                    write!(f, "\u{265C}")
                } else {
                    write!(f, "\u{2656}")
                }
            }
            PieceType::Queen => {
                if self.color == Color::White {
                    write!(f, "\u{265B}")
                } else {
                    write!(f, "\u{2655}")
                }
            }
        }
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        match piece_type {
            PieceType::Pawn => Piece { piece_type, color },
            PieceType::Knight => Piece { piece_type, color },
            PieceType::Bishop => Piece { piece_type, color },
            PieceType::Rook => Piece { piece_type, color },
            PieceType::Queen => Piece { piece_type, color },
            PieceType::King => Piece { piece_type, color },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MoveType {
    Normal,
    Jump,
    Enpassant,
    LongCastle,
    ShortCastle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Move {
    piece: Piece,
    from: Position,
    to: Position,
    move_type: MoveType,
    traversed_squares: Vec<Position>,
    captured_piece: Option<Piece>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?} -> {:?}",
            self.piece.piece_type, self.from, self.to
        )
    }
}

#[derive(Debug, Clone)]
pub enum UserInput {
    Move(Position, Position),
    Promotion(Piece, Position),
    Draw,
    Resign,
}

pub enum UserOutput {
    CheckMate,
    StaleMate,
    InvalidMove,
    Promotion(Position),
    Draw,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub turn: Color,
    board: HashMap<Position, Option<Piece>>,
    captured: HashMap<Color, Vec<Piece>>,
    history: Vec<Move>,
    number_of_repeated_board_states: HashMap<(Color, BoardArray, Vec<Move>), u8>,
    number_of_moves_without_captures_or_pawn_moves: u8,
    able_to_long_castle: HashMap<Color, bool>,
    able_to_short_castle: HashMap<Color, bool>,
    protected_squares: HashMap<Color, Vec<Position>>,
    pieces_attacking_king: HashMap<Color, Vec<(Piece, Vec<Position>)>>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str("  -");
        for _ in 1..=16 {
            res.push_str("--");
        }
        res.push('\n');

        for y in ('1'..='8').rev() {
            res.push_str(format!("{y} | ").as_str());
            for x in 'a'..='h' {
                match &self.board[&Position(x, y)] {
                    None => res.push_str("  | "),
                    Some(piece) => res.push_str(format!("{piece} | ").as_str()),
                }
            }
            res.push_str("\n".to_string().as_str());
            res.push_str("  -".to_string().as_str());
            for _ in 1..=16 {
                res.push_str("--");
            }
            res.push('\n');
        }
        res.push_str("    ");
        for x in 'a'..='h' {
            res.push_str(format!("{x}   ").as_str());
        }
        res.push('\n');
        write!(f, "{res}")
    }
}

// NOTE: all the public functions are used by the UI
impl Game {
    pub fn new() -> Game {
        let mut board: HashMap<Position, Option<Piece>> = HashMap::new();
        for x in 'a'..='h' {
            for y in '1'..='8' {
                let piece = match (x, y) {
                    (_, '2') => Some(Piece::new(PieceType::Pawn, Color::White)),
                    (_, '7') => Some(Piece::new(PieceType::Pawn, Color::Black)),
                    ('a' | 'h', '1') => Some(Piece::new(PieceType::Rook, Color::White)),
                    ('a' | 'h', '8') => Some(Piece::new(PieceType::Rook, Color::Black)),
                    ('b' | 'g', '1') => Some(Piece::new(PieceType::Knight, Color::White)),
                    ('b' | 'g', '8') => Some(Piece::new(PieceType::Knight, Color::Black)),
                    ('c' | 'f', '1') => Some(Piece::new(PieceType::Bishop, Color::White)),
                    ('c' | 'f', '8') => Some(Piece::new(PieceType::Bishop, Color::Black)),
                    ('d', '1') => Some(Piece::new(PieceType::Queen, Color::White)),
                    ('d', '8') => Some(Piece::new(PieceType::Queen, Color::Black)),
                    ('e', '1') => Some(Piece::new(PieceType::King, Color::White)),
                    ('e', '8') => Some(Piece::new(PieceType::King, Color::Black)),
                    _ => None,
                };
                board.insert(Position(x, y), piece);
            }
        }
        let captured: HashMap<Color, Vec<Piece>> = HashMap::new();
        let history: Vec<Move> = Vec::new();
        let able_to_castle = HashMap::from([(Color::White, true), (Color::Black, true)]);
        let mut protected_squares_white: Vec<Position> = Vec::new();
        let mut protected_squares_black: Vec<Position> = Vec::new();
        for x in 'a'..='h' {
            protected_squares_white.push(Position(x, '3'));
            protected_squares_white.push(Position(x, '2'));
            protected_squares_black.push(Position(x, '6'));
            protected_squares_black.push(Position(x, '7'));
            if x != 'a' || x != 'h' {
                protected_squares_white.push(Position(x, '1'));
                protected_squares_black.push(Position(x, '8'));
            }
        }

        let protected_squares = HashMap::from([
            (Color::White, protected_squares_white),
            (Color::Black, protected_squares_black),
        ]);

        let pieces_attacking_king =
            HashMap::from([(Color::White, Vec::new()), (Color::Black, Vec::new())]);

        let mut game = Game {
            turn: Color::White,
            board,
            captured,
            history,
            protected_squares,
            pieces_attacking_king,
            number_of_moves_without_captures_or_pawn_moves: 0,
            number_of_repeated_board_states: HashMap::new(),
            able_to_long_castle: able_to_castle.clone(),
            able_to_short_castle: able_to_castle,
        };

        let board_array = game.get_board_array();
        let all_possible_moves = game.get_all_possible_moves();
        let key = (game.turn.clone(), board_array, all_possible_moves);

        game.number_of_repeated_board_states.insert(key, 1);

        game
    }
    pub fn process_input(&mut self, user_input: &UserInput) -> Option<UserOutput> {
        match user_input {
            UserInput::Move(from, to) => {
                match self.get_move_if_valid(*from, *to) {
                    Some(mv) => {
                        if mv.piece.piece_type == PieceType::Pawn
                            && mv.move_type == MoveType::Normal
                            && (mv.to.1 == '8' || mv.to.1 == '1')
                        {
                            // update position
                            self.board.insert(*from, None);
                            self.board.insert(*to, Some(mv.piece.clone()));
                            self.history.push(mv.clone());
                            return Some(UserOutput::Promotion(mv.to));
                        }

                        self.turn = self.turn.invert();
                        // update position
                        self.board.insert(*from, None);
                        self.board.insert(*to, Some(mv.piece.clone()));

                        if mv.move_type == MoveType::Enpassant {
                            let direction = if mv.piece.color == Color::White {
                                1
                            } else {
                                -1
                            };
                            self.board.insert(mv.to.add((0, -direction)), None);
                        }
                        if mv.move_type == MoveType::LongCastle {
                            if mv.piece.color == Color::White {
                                self.board.insert(Position('a', '1'), None);
                                self.board.insert(
                                    Position('d', '1'),
                                    Some(Piece::new(PieceType::Rook, Color::White)),
                                );
                            } else {
                                self.board.insert(Position('a', '8'), None);
                                self.board.insert(
                                    Position('d', '8'),
                                    Some(Piece::new(PieceType::Rook, Color::Black)),
                                );
                            }
                        }
                        if mv.move_type == MoveType::ShortCastle {
                            if mv.piece.color == Color::White {
                                self.board.insert(Position('h', '1'), None);
                                self.board.insert(
                                    Position('f', '1'),
                                    Some(Piece::new(PieceType::Rook, Color::White)),
                                );
                            } else {
                                self.board.insert(Position('h', '8'), None);
                                self.board.insert(
                                    Position('f', '8'),
                                    Some(Piece::new(PieceType::Rook, Color::Black)),
                                );
                            }
                        }
                        // FIXME: circular relationship in those function. Dirty fix was used by checking bool get_protected when checking if check
                        //  get_all_protected_squares has to be run before pieces_attacking_king right now
                        self.protected_squares = self.get_all_protected_squares(true);
                        self.pieces_attacking_king = self.pieces_attacking_king(true);

                        if mv.captured_piece.is_some() {
                            self.captured
                                .entry(mv.piece.color.clone())
                                .or_default()
                                .push(mv.captured_piece.clone().unwrap());
                        }
                        if (mv.piece.piece_type == PieceType::King
                            || mv.piece.piece_type == PieceType::Rook)
                            && (self.able_to_long_castle[&mv.piece.color]
                                || self.able_to_short_castle[&mv.piece.color])
                        {
                            if mv.piece.piece_type == PieceType::King {
                                self.able_to_short_castle
                                    .insert(mv.piece.color.clone(), false);
                                self.able_to_long_castle
                                    .insert(mv.piece.color.clone(), false);
                            } else {
                                let long_caste_pos: Position = if mv.piece.color == Color::White {
                                    Position('a', '1')
                                } else {
                                    Position('a', '8')
                                };
                                let short_caste_pos: Position = if mv.piece.color == Color::White {
                                    Position('h', '1')
                                } else {
                                    Position('h', '8')
                                };
                                if mv.from == long_caste_pos {
                                    self.able_to_long_castle
                                        .insert(mv.piece.color.clone(), false);
                                } else if mv.from == short_caste_pos {
                                    self.able_to_short_castle
                                        .insert(mv.piece.color.clone(), false);
                                }
                            }
                        }

                        if !(mv.captured_piece.is_some() || mv.piece.piece_type == PieceType::Pawn)
                        {
                            self.number_of_moves_without_captures_or_pawn_moves += 1;
                        }

                        self.history.push(mv);

                        let board_array = self.get_board_array();
                        let all_possible_moves = self.get_all_possible_moves();

                        let key = (self.turn.clone(), board_array, all_possible_moves);
                        if self.number_of_repeated_board_states.contains_key(&key) {
                            let num_pos = self.number_of_repeated_board_states[&key];
                            self.number_of_repeated_board_states
                                .insert(key, num_pos + 1);
                        } else {
                            self.number_of_repeated_board_states.insert(key, 1);
                        }

                        if self.is_a_draw() {
                            return Some(UserOutput::Draw);
                        }

                        if self.no_possible_moves(&self.turn) {
                            return if self.check(&self.turn) {
                                Some(UserOutput::CheckMate)
                            } else {
                                Some(UserOutput::StaleMate)
                            };
                        }

                        None
                    }
                    None => Some(UserOutput::InvalidMove),
                }
            }
            UserInput::Promotion(piece, pos) => {
                self.turn = self.turn.invert();
                self.board.insert(*pos, Some(piece.clone()));

                // FIXME: circular relationship in those function. Dirty fix was used by checking bool get_protected when checking if check
                //  get_all_protected_squares has to be run before pieces_attacking_king right now
                self.protected_squares = self.get_all_protected_squares(true);
                self.pieces_attacking_king = self.pieces_attacking_king(true);

                if self.no_possible_moves(&self.turn) {
                    return if self.check(&self.turn) {
                        Some(UserOutput::CheckMate)
                    } else {
                        Some(UserOutput::StaleMate)
                    };
                }

                None
            }
            _ => {
                unreachable!()
            }
        }
    }
}

// NOTE: all the private functions are used by the game logic
impl Game {
    fn obstacles_in_one_move(&self, pos: Position) -> Option<Obstacle> {
        match self.board.get(&pos) {
            // out of boundary
            None => Some(Obstacle::OutOfBoundary),
            // no piece there: good
            Some(None) => None,
            // cannot be same color and if you want to get protected any piece is an obstacle
            Some(Some(p)) => Some(Obstacle::Piece(p.color.clone())),
        }
    }
    fn can_short_castle(&self, color: &Color) -> bool {
        match color {
            Color::Black => {
                !self.check(color)
                    && self.able_to_short_castle[color]
                    && self.board[&Position('f', '8')].is_none()
                    && self.board[&Position('g', '8')].is_none()
                    && !self.pos_protected(Position('f', '8'), &color.invert())
                    && !self.pos_protected(Position('g', '8'), &color.invert())
            }
            Color::White => {
                !self.check(color)
                    && self.able_to_short_castle[color]
                    && self.board[&Position('f', '1')].is_none()
                    && self.board[&Position('g', '1')].is_none()
                    && !self.pos_protected(Position('f', '1'), &color.invert())
                    && !self.pos_protected(Position('g', '1'), &color.invert())
            }
        }
    }
    fn can_long_castle(&self, color: &Color) -> bool {
        match color {
            Color::Black => {
                !self.check(color)
                    && self.able_to_long_castle[color]
                    && self.board[&Position('b', '8')].is_none()
                    && self.board[&Position('c', '8')].is_none()
                    && self.board[&Position('d', '8')].is_none()
                    && !self.pos_protected(Position('c', '8'), &color.invert())
                    && !self.pos_protected(Position('d', '8'), &color.invert())
            }
            Color::White => {
                !self.check(color)
                    && self.able_to_long_castle[color]
                    && self.board[&Position('b', '1')].is_none()
                    && self.board[&Position('c', '1')].is_none()
                    && self.board[&Position('d', '1')].is_none()
                    && !self.pos_protected(Position('c', '1'), &color.invert())
                    && !self.pos_protected(Position('d', '1'), &color.invert())
            }
        }
    }

    fn get_all_protected_squares(&self, filter_pinned: bool) -> HashMap<Color, Vec<Position>> {
        let mut protected_squares_white: Vec<Position> = Vec::new();
        let mut protected_squares_black: Vec<Position> = Vec::new();
        for x in 'a'..='h' {
            for y in '1'..='8' {
                if self.board[&Position(x, y)].is_some() {
                    let piece = self.board[&Position(x, y)].as_ref().unwrap();
                    let possible_moves = self.possible_moves(Position(x, y), true, filter_pinned);
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

        HashMap::from([
            (Color::White, protected_squares_white),
            (Color::Black, protected_squares_black),
        ])
    }
    fn pos_protected(&self, pos: Position, color: &Color) -> bool {
        let protected_positions: &Vec<Position> = if *color == Color::White {
            self.protected_squares[&Color::White].as_ref()
        } else {
            self.protected_squares[&Color::Black].as_ref()
        };
        for pos_protected in protected_positions {
            if pos == *pos_protected {
                return true;
            }
        }
        false
    }
    fn get_moves_in_one_direction<I1, I2>(
        &self,
        x_path: I1,
        y_path: I2,
        pos: Position,
        piece: &Piece,
        get_protected: bool,
    ) -> Vec<Move>
    where
        I1: Iterator<Item = i8>,
        I2: Iterator<Item = i8>,
    {
        let mut moves = Vec::new();

        let mut traversed_squares = vec![pos];
        for (x, y) in x_path.zip(y_path) {
            let new_pos = pos.add((x, y));
            let obstacle = self.obstacles_in_one_move(new_pos);

            if obstacle.is_some() {
                match obstacle.unwrap() {
                    Obstacle::OutOfBoundary => {
                        break;
                    }
                    Obstacle::Piece(obstacle_color) => {
                        if !get_protected && obstacle_color == piece.color {
                            break;
                        } else {
                            traversed_squares.push(new_pos);
                            moves.push(Move {
                                piece: piece.clone(),
                                move_type: MoveType::Normal,
                                from: pos,
                                to: new_pos,
                                traversed_squares: traversed_squares.clone(),
                                captured_piece: self.board[&new_pos].clone(),
                            });
                            break;
                        }
                    }
                }
            }

            traversed_squares.push(new_pos);
            moves.push(Move {
                piece: piece.clone(),
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: self.board[&new_pos].clone(),
            });
        }
        moves
    }

    fn possible_horizontal_vertical_moves(
        &self,
        pos: Position,
        piece: &Piece,
        get_protected: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        // horizontal movement right
        moves.append(
            self.get_moves_in_one_direction(1..=8, [0i8; 8].into_iter(), pos, piece, get_protected)
                .as_mut(),
        );

        // horizontal movement left
        moves.append(
            self.get_moves_in_one_direction(
                (-8..=-1).rev(),
                [0i8; 8].into_iter(),
                pos,
                piece,
                get_protected,
            )
            .as_mut(),
        );

        // vertical movement up
        moves.append(
            self.get_moves_in_one_direction([0i8; 8].into_iter(), 1..=8, pos, piece, get_protected)
                .as_mut(),
        );

        // vertical movement down
        moves.append(
            self.get_moves_in_one_direction(
                [0i8; 8].into_iter(),
                (-8..=-1).rev(),
                pos,
                piece,
                get_protected,
            )
            .as_mut(),
        );

        moves
    }

    fn possible_diagonal_moves(
        &self,
        pos: Position,
        piece: &Piece,
        get_protected: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        // right diagonal up
        moves.append(
            self.get_moves_in_one_direction(1..=8, 1..=8, pos, piece, get_protected)
                .as_mut(),
        );

        // right diagonal down
        moves.append(
            self.get_moves_in_one_direction(1..=8, (-8..=-1).rev(), pos, piece, get_protected)
                .as_mut(),
        );

        // left diagonal up
        moves.append(
            self.get_moves_in_one_direction((-8..=-1).rev(), 1..=8, pos, piece, get_protected)
                .as_mut(),
        );

        // left diagonal down
        moves.append(
            self.get_moves_in_one_direction(
                (-8..=-1).rev(),
                (-8..=-1).rev(),
                pos,
                piece,
                get_protected,
            )
            .as_mut(),
        );

        moves
    }
    fn pieces_attacking_king(
        &self,
        filter_pinned: bool,
    ) -> HashMap<Color, Vec<(Piece, Vec<Position>)>> {
        let mut pieces_white: Vec<(Piece, Vec<Position>)> = Vec::new();
        let mut pieces_black: Vec<(Piece, Vec<Position>)> = Vec::new();
        for x in 'a'..='h' {
            for y in '1'..='8' {
                let moves = self.possible_moves(Position(x, y), false, filter_pinned);
                for mv in moves {
                    if mv.captured_piece.is_some()
                        && mv.captured_piece.unwrap().piece_type == PieceType::King
                    {
                        if mv.piece.color == Color::White {
                            pieces_white.push((mv.piece, mv.traversed_squares));
                        } else {
                            pieces_black.push((mv.piece, mv.traversed_squares));
                        }
                    }
                }
            }
        }

        HashMap::from([(Color::White, pieces_black), (Color::Black, pieces_white)])
    }

    fn no_possible_moves(&self, color: &Color) -> bool {
        for x in 'a'..='h' {
            for y in '1'..='8' {
                let moves = self.possible_moves(Position(x, y), false, false);
                if !moves.is_empty() && moves[0].piece.color == *color {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    fn check(&self, color: &Color) -> bool {
        !self.pieces_attacking_king[color].is_empty()
    }

    fn possible_moves(&self, pos: Position, get_protected: bool, filter_pinned: bool) -> Vec<Move> {
        if self.board[&pos].is_none() {
            return Vec::new();
        }

        let piece = self.board[&pos].as_ref().unwrap();

        if !get_protected
            && self.check(&piece.color)
            && self.pieces_attacking_king[&piece.color].len() > 1
            && piece.piece_type != PieceType::King
        {
            // double check: only king can move
            return Vec::new();
        }

        let mut moves: Vec<Move> = Vec::new();

        match piece.piece_type {
            PieceType::King => {
                for (x, y) in [
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                ] {
                    let new_pos = pos.add((x, y));
                    match self.obstacles_in_one_move(new_pos) {
                        None => {
                            if get_protected || !self.pos_protected(new_pos, &piece.color.invert())
                            {
                                moves.push(Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: self.board[&new_pos].clone(),
                                });
                            }
                        }
                        Some(Obstacle::Piece(obstacle_color)) => {
                            if get_protected
                                || (!self.pos_protected(new_pos, &piece.color.invert())
                                    && piece.color != obstacle_color)
                            {
                                moves.push(Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: self.board[&new_pos].clone(),
                                });
                            }
                        }
                        Some(Obstacle::OutOfBoundary) => {
                            // nothing to do
                        }
                    }
                }

                if !get_protected && self.can_long_castle(&piece.color) {
                    moves.push(Move {
                        piece: piece.clone(),
                        move_type: MoveType::LongCastle,
                        from: pos,
                        to: pos.add((-2, 0)),
                        traversed_squares: vec![pos, pos.add((-1, 0)), pos.add((-2, 0))],
                        captured_piece: self.board[&pos.add((-2, 0))].clone(),
                    });
                }
                if !get_protected && self.can_short_castle(&piece.color) {
                    moves.push(Move {
                        piece: piece.clone(),
                        move_type: MoveType::ShortCastle,
                        from: pos,
                        to: pos.add((2, 0)),
                        traversed_squares: vec![pos, pos.add((1, 0)), pos.add((2, 0))],
                        captured_piece: self.board[&pos.add((2, 0))].clone(),
                    });
                }
            }

            PieceType::Queen => {
                moves.append(
                    self.possible_horizontal_vertical_moves(pos, piece, get_protected)
                        .as_mut(),
                );

                moves.append(
                    self.possible_diagonal_moves(pos, piece, get_protected)
                        .as_mut(),
                );
            }

            PieceType::Rook => {
                moves.append(
                    self.possible_horizontal_vertical_moves(pos, piece, get_protected)
                        .as_mut(),
                );
            }

            PieceType::Bishop => {
                moves.append(
                    self.possible_diagonal_moves(pos, piece, get_protected)
                        .as_mut(),
                );
            }

            PieceType::Knight => {
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
                    let new_pos = pos.add(pos_int);
                    match self.obstacles_in_one_move(new_pos) {
                        None => {
                            moves.push(Move {
                                piece: piece.clone(),
                                move_type: MoveType::Jump,
                                from: pos,
                                to: new_pos,
                                traversed_squares: vec![pos, new_pos],
                                captured_piece: self.board[&new_pos].clone(),
                            });
                        }
                        Some(Obstacle::Piece(obstacle_color)) => {
                            if get_protected || piece.color != obstacle_color {
                                moves.push(Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Jump,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: self.board[&new_pos].clone(),
                                });
                            }
                        }
                        Some(Obstacle::OutOfBoundary) => {
                            // nothing to do
                        }
                    }
                }
            }

            PieceType::Pawn => {
                let (direction, rel_pos_to_iter) = if piece.color == Color::White {
                    (1, if pos.1 == '2' { 1..3 } else { 1..2 })
                } else {
                    (-1, if pos.1 == '7' { 1..3 } else { 1..2 })
                };

                // Normal moves
                if !get_protected {
                    for y in rel_pos_to_iter {
                        let new_pos = pos.add((0, direction * y));
                        match self.board.get(&new_pos) {
                            Some(None) => {
                                moves.push(Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: self.board[&new_pos].clone(),
                                });
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }

                // check if able to capture a piece
                for new_pos_int in [(1, direction), (-1, direction)] {
                    let new_pos = pos.add(new_pos_int);
                    match self.board.get(&new_pos) {
                        Some(Some(other_piece)) => {
                            if get_protected || other_piece.color != piece.color {
                                moves.push(Move {
                                    piece: piece.clone(),
                                    move_type: MoveType::Normal,
                                    from: pos,
                                    to: new_pos,
                                    traversed_squares: vec![pos, new_pos],
                                    captured_piece: self.board[&new_pos].clone(),
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
                                    captured_piece: self.board[&new_pos].clone(),
                                });
                            }
                        }
                        None => {} // do nothing
                    }
                }

                // Enpassant
                if !get_protected && !self.history.is_empty() {
                    let last_move = self.history.last().unwrap();

                    if last_move.piece.piece_type == PieceType::Pawn
                        && (last_move.from.1 as i8 - last_move.to.1 as i8).abs() == 2
                        && (last_move.to == pos.add((1, 0)) || last_move.to == pos.add((-1, 0)))
                    {
                        let new_pos = if last_move.to == pos.add((1, 0)) {
                            pos.add((1, direction))
                        } else {
                            pos.add((-1, direction))
                        };
                        moves.push(Move {
                            piece: piece.clone(),
                            move_type: MoveType::Enpassant,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[&new_pos.add((0, -direction))].clone(),
                        });
                    }
                }
            }
        }

        if !get_protected && self.check(&piece.color) && piece.piece_type != PieceType::King {
            // filter out moves that do not protect the king. Only for pieces other than the king.
            debug_assert_eq!(
                1,
                self.pieces_attacking_king[&piece.color].len(),
                "More than one piece attacking the king"
            );
            let (_, pos) = self.pieces_attacking_king[&piece.color][0].clone();
            moves = moves.drain(..).filter(|x| pos.contains(&x.to)).collect();
        }

        if filter_pinned {
            moves = moves
                .drain(..)
                .filter(|x| self.piece_is_not_pinned(x))
                .collect();
        }

        moves
    }
    fn piece_is_not_pinned(&self, mv: &Move) -> bool {
        if mv.piece.piece_type == PieceType::King {
            true
        } else {
            let mut game_after_move = self.clone();
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
                game_after_move
                    .board
                    .insert(mv.to.add((0, -direction)), None);
            }
            game_after_move.protected_squares = game_after_move.get_all_protected_squares(false);
            game_after_move.pieces_attacking_king = game_after_move.pieces_attacking_king(false);
            game_after_move.pieces_attacking_king[&mv.piece.color].is_empty()
        }
    }

    fn get_move_if_valid(&self, from: Position, to: Position) -> Option<Move> {
        match self.board.get(&from) {
            Some(Some(piece)) => {
                if self.turn == piece.color {
                    let matching_moves: Vec<Move> = self
                        .possible_moves(from, false, true)
                        .drain(..)
                        .filter(|x| x.to == to)
                        .collect();
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
            _ => None,
        }
    }
    fn get_board_array(&self) -> BoardArray {
        const INIT: Option<Piece> = None;
        let mut piece_array = [INIT; 64];
        let mut idx = 0;
        for x in 'a'..='h' {
            for y in '1'..='8' {
                piece_array[idx] = self.board[&Position(x, y)].clone();
                idx += 1;
            }
        }
        piece_array
    }

    fn get_all_possible_moves(&self) -> Vec<Move> {
        let mut all_possible_moves: Vec<Move> = Vec::new();
        for x in 'a'..='h' {
            for y in '1'..='8' {
                if self.board[&Position(x, y)].is_some() {
                    all_possible_moves
                        .append(self.possible_moves(Position(x, y), true, true).as_mut());
                }
            }
        }
        all_possible_moves
    }
    fn is_a_draw(&self) -> bool {
        if self.number_of_moves_without_captures_or_pawn_moves >= 50 {
            true
        } else {
            self.number_of_repeated_board_states
                .clone()
                .into_iter()
                .filter(|(_, num)| *num >= 3)
                .peekable()
                .peek()
                .is_some()
        }
    }
}
