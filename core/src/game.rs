use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::sync::Mutex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}
pub const COLOR_COUNT: usize = 2;

impl Color {
    #[must_use]
    pub const fn invert(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    King,
    Rook,
    Queen,
}

impl PieceType {
    pub const fn value(&self) -> u8 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight | PieceType::Bishop => 3,
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
    fn add(self, to_add: (i8, i8)) -> Position {
        fn checked_add(a: char, b: i8) -> char {
            let b_abs = b.unsigned_abs();
            let a_u8 = a as u8;
            if b < 0 {
                a_u8.saturating_sub(b_abs) as char
            } else {
                a_u8.saturating_add(b_abs) as char
            }
        }
        let new_x = checked_add(self.0, to_add.0);
        let new_y = checked_add(self.1, to_add.1);

        Position(new_x, new_y)
    }

    #[inline]
    pub fn as_index(self: Position) -> usize {
        let x = self.0 as u8 - b'a';
        let y = self.1 as u8 - b'1';
        (y as usize) * BOARD_SIZE + (x as usize)
    }

    #[inline]
    fn try_as_index(self: Position) -> Option<usize> {
        if self.0 < 'a' || self.0 > 'h' || self.1 < '1' || self.1 > '8' {
            return None;
        }
        Some(self.as_index())
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
impl TryFrom<usize> for Position {
    type Error = ();

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        if val >= TOTAL_SQUARES {
            return Err(());
        }
        let x = (val % BOARD_SIZE) as u8 + b'a';
        let y = (val / BOARD_SIZE) as u8 + b'1';
        Ok(Position(x as char, y as char))
    }
}

pub const BOARD_SIZE: usize = 8;
pub const TOTAL_SQUARES: usize = BOARD_SIZE * BOARD_SIZE;
type Board = [Option<Piece>; TOTAL_SQUARES];

const fn all_possibles_sqares() -> [(char, char); TOTAL_SQUARES] {
    let mut squares = [('a', 'a'); 64];
    let mut i: usize = 0;
    while i < BOARD_SIZE {
        let mut j: usize = 0;
        while j < BOARD_SIZE {
            squares[i * BOARD_SIZE + j] =
                (('a' as u8 + i as u8) as char, ('1' as u8 + j as u8) as char);
            j += 1;
        }
        i += 1;
    }
    squares
}
const ALL_POSSIBLE_SQUARES: [(char, char); TOTAL_SQUARES] = all_possibles_sqares();

const fn horizontal_directions() -> [([i8; BOARD_SIZE], [i8; BOARD_SIZE]); 4] {
    let mut forward = [0; BOARD_SIZE];
    let mut backward = [0; BOARD_SIZE];
    let mut i = 0;
    while i < BOARD_SIZE {
        forward[i] = i as i8 + 1;
        backward[i] = -(i as i8 + 1);
        i += 1;
    }
    [
        (forward, [0i8; BOARD_SIZE]),  // horizontal right
        (backward, [0i8; BOARD_SIZE]), // horizontal left
        ([0i8; BOARD_SIZE], forward),  // vertical up
        ([0i8; BOARD_SIZE], backward), // vertical down
    ]
}
const HORIZONTAL_DIRECTIONS: [([i8; BOARD_SIZE], [i8; BOARD_SIZE]); 4] = horizontal_directions();
const fn diagonal_directions() -> [([i8; BOARD_SIZE], [i8; BOARD_SIZE]); 4] {
    let mut forward = [0; BOARD_SIZE];
    let mut backward = [0; BOARD_SIZE];
    let mut i = 0;
    while i < BOARD_SIZE {
        forward[i] = i as i8 + 1;
        backward[i] = -(i as i8 + 1);
        i += 1;
    }
    [
        (forward, forward),   // right diagonal up
        (forward, backward),  // right diagonal down
        (backward, forward),  // left diagonal up
        (backward, backward), // left diagonal down
    ]
}
const DIAGONAL_DIRECTIONS: [([i8; BOARD_SIZE], [i8; BOARD_SIZE]); 4] = diagonal_directions();

const fn queen_directions() -> [([i8; BOARD_SIZE], [i8; BOARD_SIZE]); 8] {
    let mut directions = [([0i8; BOARD_SIZE], [0i8; BOARD_SIZE]);
        HORIZONTAL_DIRECTIONS.len() + DIAGONAL_DIRECTIONS.len()];
    let mut i = 0;
    while i < HORIZONTAL_DIRECTIONS.len() {
        directions[i] = HORIZONTAL_DIRECTIONS[i];
        i += 1;
    }
    while i < directions.len() {
        directions[i] = DIAGONAL_DIRECTIONS[i - HORIZONTAL_DIRECTIONS.len()];
        i += 1;
    }
    directions
}
pub const QUEEN_DIRECTIONS: [([i8; BOARD_SIZE], [i8; BOARD_SIZE]);
    HORIZONTAL_DIRECTIONS.len() + DIAGONAL_DIRECTIONS.len()] = queen_directions();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Obstacle {
    OutOfBoundary,
    Piece(Color),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
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
    #[must_use]
    pub const fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum MoveType {
    Normal,
    Jump,
    Enpassant,
    LongCastle,
    ShortCastle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub captured_piece: Option<Piece>,
    move_type: MoveType,
    traversed_squares: Vec<Position>,
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

#[derive(Debug, Clone)]
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
    pub board: Board,
    pub captured: [Vec<Piece>; COLOR_COUNT],
    history: Vec<Move>,
    number_of_repeated_board_states: HashMap<(Color, Board, Vec<Move>), u8>,
    number_of_moves_without_captures_or_pawn_moves: u8,
    able_to_long_castle: [bool; COLOR_COUNT],
    able_to_short_castle: [bool; COLOR_COUNT],
    protected_squares: [Vec<Position>; COLOR_COUNT],
    pieces_attacking_king: [Vec<(Piece, Vec<Position>)>; COLOR_COUNT],
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
                match &self.board[Position(x, y).as_index()] {
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

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

// NOTE: all the public functions are used by the UI
impl Game {
    #[must_use]
    pub fn new() -> Game {
        let mut board: Board = [None; TOTAL_SQUARES];
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
                board[Position(x, y).as_index()] = piece;
            }
        }
        let captured = [Vec::new(), Vec::new()];
        let history = Vec::new();
        let able_to_castle = [true, true];
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

        let protected_squares = [protected_squares_white, protected_squares_black];

        let pieces_attacking_king = [Vec::new(), Vec::new()];

        let mut game = Game {
            turn: Color::White,
            board,
            captured,
            history,
            protected_squares,
            pieces_attacking_king,
            number_of_moves_without_captures_or_pawn_moves: 0,
            number_of_repeated_board_states: HashMap::new(),
            able_to_long_castle: able_to_castle,
            able_to_short_castle: able_to_castle,
        };

        let board = game.board;
        let all_possible_moves = game.get_all_possible_moves();
        let key = (game.turn, board, all_possible_moves);

        game.number_of_repeated_board_states.insert(key, 1);

        game
    }

    #[allow(clippy::too_many_lines)]
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
                            self.board[from.as_index()] = None;
                            self.board[to.as_index()] = Some(mv.piece);
                            self.history.push(mv.clone());
                            return Some(UserOutput::Promotion(mv.to));
                        }

                        self.turn = self.turn.invert();
                        // update position
                        self.board[from.as_index()] = None;
                        self.board[to.as_index()] = Some(mv.piece);

                        if mv.move_type == MoveType::Enpassant {
                            let direction = if mv.piece.color == Color::White {
                                1
                            } else {
                                -1
                            };
                            self.board[mv.to.add((0, -direction)).as_index()] = None;
                        }
                        if mv.move_type == MoveType::LongCastle {
                            if mv.piece.color == Color::White {
                                self.board[Position('a', '1').as_index()] = None;
                                self.board[Position('d', '1').as_index()] =
                                    Some(Piece::new(PieceType::Rook, Color::White));
                            } else {
                                self.board[Position('a', '8').as_index()] = None;
                                self.board[Position('d', '8').as_index()] =
                                    Some(Piece::new(PieceType::Rook, Color::Black));
                            }
                        }
                        if mv.move_type == MoveType::ShortCastle {
                            if mv.piece.color == Color::White {
                                self.board[Position('h', '1').as_index()] = None;
                                self.board[Position('f', '1').as_index()] =
                                    Some(Piece::new(PieceType::Rook, Color::White));
                            } else {
                                self.board[Position('h', '8').as_index()] = None;
                                self.board[Position('f', '8').as_index()] =
                                    Some(Piece::new(PieceType::Rook, Color::Black));
                            }
                        }
                        // FIXME: circular relationship in those function. Dirty fix was used by checking bool get_protected when checking if check
                        //  get_all_protected_squares has to be run before pieces_attacking_king right now
                        self.protected_squares = self.get_all_protected_squares(true);
                        self.pieces_attacking_king = self.pieces_attacking_king(true);

                        if let Some(captured_piece) = mv.captured_piece {
                            self.captured[mv.piece.color as usize].push(captured_piece);
                        }
                        if (mv.piece.piece_type == PieceType::King
                            || mv.piece.piece_type == PieceType::Rook)
                            && (self.able_to_long_castle[mv.piece.color as usize]
                                || self.able_to_short_castle[mv.piece.color as usize])
                        {
                            if mv.piece.piece_type == PieceType::King {
                                self.able_to_short_castle[mv.piece.color as usize] = false;
                                self.able_to_long_castle[mv.piece.color as usize] = false;
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
                                    self.able_to_long_castle[mv.piece.color as usize] = false;
                                } else if mv.from == short_caste_pos {
                                    self.able_to_short_castle[mv.piece.color as usize] = false;
                                }
                            }
                        }

                        if !(mv.captured_piece.is_some() || mv.piece.piece_type == PieceType::Pawn)
                        {
                            self.number_of_moves_without_captures_or_pawn_moves += 1;
                        } else {
                            self.number_of_moves_without_captures_or_pawn_moves = 0;
                        }

                        self.history.push(mv);

                        let board = self.board;
                        let all_possible_moves = self.get_all_possible_moves();

                        let key = (self.turn, board, all_possible_moves);
                        if self.number_of_repeated_board_states.contains_key(&key) {
                            let num_pos = self.number_of_repeated_board_states[&key];
                            self.number_of_repeated_board_states
                                .insert(key, num_pos + 1);
                        } else {
                            self.number_of_repeated_board_states.insert(key, 1);
                        }

                        if self.no_possible_moves(self.turn) {
                            return if self.check(self.turn) {
                                Some(UserOutput::CheckMate)
                            } else {
                                Some(UserOutput::StaleMate)
                            };
                        }

                        if self.is_a_draw() {
                            return Some(UserOutput::Draw);
                        }

                        None
                    }
                    None => Some(UserOutput::InvalidMove),
                }
            }
            UserInput::Promotion(piece, pos) => {
                self.turn = self.turn.invert();
                self.board[pos.as_index()] = Some(*piece);

                // FIXME: circular relationship in those function. Dirty fix was used by checking bool get_protected when checking if check
                //  get_all_protected_squares has to be run before pieces_attacking_king right now
                self.protected_squares = self.get_all_protected_squares(true);
                self.pieces_attacking_king = self.pieces_attacking_king(true);

                if self.no_possible_moves(self.turn) {
                    return if self.check(self.turn) {
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

    #[must_use]
    pub fn get_all_currently_valid_moves(&self) -> Vec<Move> {
        let all_possible_moves = ALL_POSSIBLE_SQUARES.par_iter().flat_map(|(x, y)| {
            let mut all_possible_moves = Vec::new();
            if let Some(piece) = &self.board[Position(*x, *y).as_index()] {
                if piece.color == self.turn {
                    all_possible_moves = self.get_valid_moves(Position(*x, *y));
                }
            }
            all_possible_moves
        });
        all_possible_moves.collect()
    }

    #[must_use]
    pub fn get_valid_moves(&self, pos: Position) -> Vec<Move> {
        self.possible_moves(pos, false, true)
    }

    #[inline]
    pub fn check(&self, color: Color) -> bool {
        !self.pieces_attacking_king[color as usize].is_empty()
    }
}

// NOTE: all the private functions are used by the game logic
impl Game {
    fn obstacles_in_one_move(&self, pos: Position) -> Option<Obstacle> {
        let Some(index) = pos.try_as_index() else {
            return Some(Obstacle::OutOfBoundary);
        };
        match self.board[index] {
            // no piece there: good
            None => None,
            // cannot be same color and if you want to get protected any piece is an obstacle
            Some(p) => Some(Obstacle::Piece(p.color)),
        }
    }

    fn can_short_castle(&self, color: Color) -> bool {
        match color {
            Color::Black => {
                !self.check(color)
                    && self.able_to_short_castle[color as usize]
                    && self.board[Position('f', '8').as_index()].is_none()
                    && self.board[Position('g', '8').as_index()].is_none()
                    && !self.pos_protected(Position('f', '8'), color.invert())
                    && !self.pos_protected(Position('g', '8'), color.invert())
            }
            Color::White => {
                !self.check(color)
                    && self.able_to_short_castle[color as usize]
                    && self.board[Position('f', '1').as_index()].is_none()
                    && self.board[Position('g', '1').as_index()].is_none()
                    && !self.pos_protected(Position('f', '1'), color.invert())
                    && !self.pos_protected(Position('g', '1'), color.invert())
            }
        }
    }

    fn can_long_castle(&self, color: Color) -> bool {
        match color {
            Color::Black => {
                !self.check(color)
                    && self.able_to_long_castle[color as usize]
                    && self.board[Position('b', '8').as_index()].is_none()
                    && self.board[Position('c', '8').as_index()].is_none()
                    && self.board[Position('d', '8').as_index()].is_none()
                    && !self.pos_protected(Position('c', '8'), color.invert())
                    && !self.pos_protected(Position('d', '8'), color.invert())
            }
            Color::White => {
                !self.check(color)
                    && self.able_to_long_castle[color as usize]
                    && self.board[Position('b', '1').as_index()].is_none()
                    && self.board[Position('c', '1').as_index()].is_none()
                    && self.board[Position('d', '1').as_index()].is_none()
                    && !self.pos_protected(Position('c', '1'), color.invert())
                    && !self.pos_protected(Position('d', '1'), color.invert())
            }
        }
    }

    fn get_all_protected_squares(&self, filter_pinned: bool) -> [Vec<Position>; COLOR_COUNT] {
        let protected_squares_white = Mutex::new(Vec::new());
        let protected_squares_black = Mutex::new(Vec::new());
        protected_squares_white.lock().unwrap().reserve(64);
        protected_squares_black.lock().unwrap().reserve(64);
        ALL_POSSIBLE_SQUARES.par_iter().for_each(|(x, y)| {
            if let Some(piece) = &self.board[Position(*x, *y).as_index()] {
                let possible_moves = self.possible_moves(Position(*x, *y), true, filter_pinned);
                for m in possible_moves {
                    if piece.color == Color::White {
                        protected_squares_white.lock().unwrap().push(m.to);
                    } else {
                        protected_squares_black.lock().unwrap().push(m.to);
                    }
                }
            }
        });

        [
            protected_squares_white.into_inner().unwrap(),
            protected_squares_black.into_inner().unwrap(),
        ]
    }

    fn pos_protected(&self, pos: Position, color: Color) -> bool {
        for pos_protected in self.protected_squares[color as usize].iter() {
            if pos == *pos_protected {
                return true;
            }
        }
        false
    }

    fn get_moves_in_one_direction(
        &self,
        x_path: &[i8],
        y_path: &[i8],
        pos: Position,
        piece: Piece,
        get_protected: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        let mut traversed_squares = vec![pos];
        for (x, y) in x_path.into_iter().zip(y_path) {
            let new_pos = pos.add((*x, *y));
            let obstacle = self.obstacles_in_one_move(new_pos);

            if let Some(obstacle) = obstacle {
                match obstacle {
                    Obstacle::Piece(obstacle_color)
                        if get_protected || obstacle_color != piece.color =>
                    {
                        traversed_squares.push(new_pos);
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Normal,
                            from: pos,
                            to: new_pos,
                            traversed_squares: traversed_squares.clone(),
                            captured_piece: self.board[new_pos.as_index()],
                        });
                    }
                    _ => {}
                }
                // if there is an obstacle we cannot go further:
                break;
            }

            traversed_squares.push(new_pos);
            moves.push(Move {
                piece,
                move_type: MoveType::Normal,
                from: pos,
                to: new_pos,
                traversed_squares: traversed_squares.clone(),
                captured_piece: self.board[new_pos.as_index()],
            });
        }
        moves
    }

    fn possible_horizontal_vertical_moves(
        &self,
        pos: Position,
        piece: Piece,
        get_protected: bool,
    ) -> Vec<Move> {
        HORIZONTAL_DIRECTIONS
            .par_iter() // Convert to a parallel iterator
            .flat_map(|(x_range, y_range)| {
                self.get_moves_in_one_direction(x_range, y_range, pos, piece, get_protected)
            })
            .collect()
    }

    fn possible_diagonal_moves(
        &self,
        pos: Position,
        piece: Piece,
        get_protected: bool,
    ) -> Vec<Move> {
        DIAGONAL_DIRECTIONS
            .par_iter() // Convert to a parallel iterator
            .flat_map(|(x_range, y_range)| {
                self.get_moves_in_one_direction(x_range, y_range, pos, piece, get_protected)
            })
            .collect()
    }

    fn pieces_attacking_king(
        &self,
        filter_pinned: bool,
    ) -> [Vec<(Piece, Vec<Position>)>; COLOR_COUNT] {
        let pieces_attacking_white = Mutex::new(Vec::new());
        let pieces_attacking_black = Mutex::new(Vec::new());
        pieces_attacking_white.lock().unwrap().reserve(16);
        pieces_attacking_black.lock().unwrap().reserve(16);
        ALL_POSSIBLE_SQUARES.par_iter().for_each(|(x, y)| {
            let moves = self.possible_moves(Position(*x, *y), false, filter_pinned);
            for mv in moves {
                if let Some(piece) = mv.captured_piece {
                    if piece.piece_type == PieceType::King {
                        if mv.piece.color == Color::White {
                            pieces_attacking_black
                                .lock()
                                .unwrap()
                                .push((mv.piece, mv.traversed_squares));
                        } else {
                            pieces_attacking_white
                                .lock()
                                .unwrap()
                                .push((mv.piece, mv.traversed_squares));
                        }
                    }
                }
            }
        });

        [
            pieces_attacking_white.into_inner().unwrap(),
            pieces_attacking_black.into_inner().unwrap(),
        ]
    }

    fn no_possible_moves(&self, color: Color) -> bool {
        for x in 'a'..='h' {
            for y in '1'..='8' {
                let moves = self.possible_moves(Position(x, y), false, true);
                if !moves.is_empty() && moves[0].piece.color == color {
                    return false;
                }
            }
        }
        true
    }

    fn possbile_pawn_moves(&self, pos: Position, piece: Piece, get_protected: bool) -> Vec<Move> {
        debug_assert_eq!(piece.piece_type, PieceType::Pawn);

        let mut moves = Vec::new();

        let (direction, rel_pos_to_iter) = if piece.color == Color::White {
            (1, if pos.1 == '2' { 1..3 } else { 1..2 })
        } else {
            (-1, if pos.1 == '7' { 1..3 } else { 1..2 })
        };

        // Normal moves
        if !get_protected {
            for y in rel_pos_to_iter {
                let new_pos = pos.add((0, direction * y));
                match self.obstacles_in_one_move(new_pos) {
                    None => {
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Normal,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[new_pos.as_index()],
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
            match self.obstacles_in_one_move(new_pos) {
                Some(Obstacle::Piece(other_piece_color)) => {
                    if get_protected || other_piece_color != piece.color {
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Normal,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[new_pos.as_index()],
                        });
                    }
                }
                None => {
                    if get_protected {
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Normal,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[new_pos.as_index()],
                        });
                    }
                }
                Some(Obstacle::OutOfBoundary) => {} // do nothing
            }
        }

        // Enpassant
        if !get_protected {
            if let Some(last_move) = self.history.last() {
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
                        piece,
                        move_type: MoveType::Enpassant,
                        from: pos,
                        to: new_pos,
                        traversed_squares: vec![pos, new_pos],
                        captured_piece: self.board[new_pos.add((0, -direction)).as_index()],
                    });
                }
            }
        }

        moves
    }

    fn possible_knight_moves(&self, pos: Position, piece: Piece, get_protected: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let dxdys: [(i8, i8); 8] = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
        ];

        for dxdy in dxdys {
            let new_pos = pos.add(dxdy);
            match self.obstacles_in_one_move(new_pos) {
                None => {
                    moves.push(Move {
                        piece,
                        move_type: MoveType::Jump,
                        from: pos,
                        to: new_pos,
                        traversed_squares: vec![pos, new_pos],
                        captured_piece: self.board[new_pos.as_index()],
                    });
                }
                Some(Obstacle::Piece(obstacle_color)) => {
                    if get_protected || piece.color != obstacle_color {
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Jump,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[new_pos.as_index()],
                        });
                    }
                }
                Some(Obstacle::OutOfBoundary) => {
                    // nothing to do
                }
            }
        }
        moves
    }

    fn possible_queen_moves(&self, pos: Position, piece: Piece, get_protected: bool) -> Vec<Move> {
        QUEEN_DIRECTIONS
            .par_iter() // Convert to a parallel iterator
            .flat_map(|(x_range, y_range)| {
                self.get_moves_in_one_direction(x_range, y_range, pos, piece, get_protected)
            })
            .collect()
    }

    fn possible_king_moves(&self, pos: Position, piece: Piece, get_protected: bool) -> Vec<Move> {
        let mut moves = Vec::with_capacity(8);
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
                    if get_protected || !self.pos_protected(new_pos, piece.color.invert()) {
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Normal,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[new_pos.as_index()],
                        });
                    }
                }
                Some(Obstacle::Piece(obstacle_color)) => {
                    if get_protected
                        || (!self.pos_protected(new_pos, piece.color.invert())
                            && piece.color != obstacle_color)
                    {
                        moves.push(Move {
                            piece,
                            move_type: MoveType::Normal,
                            from: pos,
                            to: new_pos,
                            traversed_squares: vec![pos, new_pos],
                            captured_piece: self.board[new_pos.as_index()],
                        });
                    }
                }
                Some(Obstacle::OutOfBoundary) => {
                    // nothing to do
                }
            }
        }

        if !get_protected && self.can_long_castle(piece.color) {
            moves.push(Move {
                piece,
                move_type: MoveType::LongCastle,
                from: pos,
                to: pos.add((-2, 0)),
                traversed_squares: vec![pos, pos.add((-1, 0)), pos.add((-2, 0))],
                captured_piece: self.board[pos.add((-2, 0)).as_index()],
            });
        }
        if !get_protected && self.can_short_castle(piece.color) {
            moves.push(Move {
                piece,
                move_type: MoveType::ShortCastle,
                from: pos,
                to: pos.add((2, 0)),
                traversed_squares: vec![pos, pos.add((1, 0)), pos.add((2, 0))],
                captured_piece: self.board[pos.add((2, 0)).as_index()],
            });
        }
        moves
    }

    fn possible_moves(&self, pos: Position, get_protected: bool, filter_pinned: bool) -> Vec<Move> {
        let Some(piece) = self.board[pos.as_index()] else {
            // no piece there -> no moves
            return Vec::new();
        };

        if !get_protected
            && self.check(piece.color)
            && self.pieces_attacking_king[piece.color as usize].len() > 1
            && piece.piece_type != PieceType::King
        {
            // double check: only king can move
            return Vec::new();
        }

        let mut moves = match piece.piece_type {
            PieceType::King => self.possible_king_moves(pos, piece, get_protected),

            PieceType::Queen => self.possible_queen_moves(pos, piece, get_protected),

            PieceType::Rook => self.possible_horizontal_vertical_moves(pos, piece, get_protected),

            PieceType::Bishop => self.possible_diagonal_moves(pos, piece, get_protected),

            PieceType::Knight => self.possible_knight_moves(pos, piece, get_protected),

            PieceType::Pawn => self.possbile_pawn_moves(pos, piece, get_protected),
        };

        if !get_protected && self.check(piece.color) && piece.piece_type != PieceType::King {
            // filter out moves that do not protect the king. Only for pieces other than the king.
            debug_assert_eq!(
                1,
                self.pieces_attacking_king[piece.color as usize].len(),
                "More than one piece attacking the king"
            );
            let (_, pos) = self.pieces_attacking_king[piece.color as usize][0].clone();
            moves = moves
                .into_par_iter()
                .filter(|x| pos.contains(&x.to))
                .collect();
        }

        if filter_pinned {
            moves = moves
                .into_par_iter()
                .filter(|x| self.piece_is_not_pinned(x))
                .collect();
        }

        moves
    }

    fn piece_is_not_pinned(&self, mv: &Move) -> bool {
        // NOTE: We also consider the King here such that he does not move into a check
        // for example when the King moves in the same direction as the line of attack of a Rook
        let mut game_after_move = self.clone();
        game_after_move.turn = game_after_move.turn.invert();
        // update position
        game_after_move.board[mv.from.as_index()] = None;
        game_after_move.board[mv.to.as_index()] = Some(mv.piece);
        if mv.move_type == MoveType::Enpassant {
            let direction = if mv.piece.color == Color::White {
                1
            } else {
                -1
            };
            game_after_move.board[mv.to.add((0, -direction)).as_index()] = None;
        }
        game_after_move.protected_squares = game_after_move.get_all_protected_squares(false);
        game_after_move.pieces_attacking_king = game_after_move.pieces_attacking_king(false);
        game_after_move.pieces_attacking_king[mv.piece.color as usize].is_empty()
    }

    fn get_move_if_valid(&self, from: Position, to: Position) -> Option<Move> {
        match self.obstacles_in_one_move(from) {
            Some(Obstacle::Piece(piece_color)) => {
                if self.turn == piece_color {
                    let matching_moves: Vec<Move> = self
                        .possible_moves(from, false, true)
                        .into_par_iter()
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

    fn get_all_possible_moves(&self) -> Vec<Move> {
        let all_possible_moves = ALL_POSSIBLE_SQUARES.par_iter().flat_map(|(x, y)| {
            let mut all_possible_moves = Vec::new();
            if self.board[Position(*x, *y).as_index()].is_some() {
                all_possible_moves = self.possible_moves(Position(*x, *y), true, true)
            }
            all_possible_moves
        });
        all_possible_moves.collect()
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
