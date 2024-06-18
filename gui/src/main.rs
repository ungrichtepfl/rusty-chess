use rand::Rng;
use raylib::prelude::*;
use rusty_chess_core::game::Color as ChessColor;
use rusty_chess_core::game::Game;
use rusty_chess_core::game::Piece;
use rusty_chess_core::game::PieceType;
use rusty_chess_core::game::UserInput;
use rusty_chess_core::game::UserOutput;
use rusty_chess_core::game::BOARD_SIZE;
use rusty_chess_core::game::TOTAL_SQUARES;
use std::path::Path;
use std::thread::available_parallelism;

const WINDOW_SIZE: i32 = 640;
const RECT_SIZE: i32 = WINDOW_SIZE / BOARD_SIZE as i32;
const TITLE: &str = "Rusty Chess";

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");

const ASSETS_PATH: &str = "assets";

const BISHOP_B: &str = "bishop-b.png";
const BISHOP_W: &str = "bishop-w.png";
const KING_B: &str = "king-b.png";
const KING_W: &str = "king-w.png";
const KNIGHT_B: &str = "knight-b.png";
const KNIGHT_W: &str = "knight-w.png";
const PAWN_B: &str = "pawn-b.png";
const PAWN_W: &str = "pawn-w.png";
const QUEEN_B: &str = "queen-b.png";
const QUEEN_W: &str = "queen-w.png";
const ROOK_B: &str = "rook-b.png";
const ROOK_W: &str = "rook-w.png";

struct Assets {
    bishop_b: Texture2D,
    bishop_w: Texture2D,
    king_b: Texture2D,
    king_w: Texture2D,
    knight_b: Texture2D,
    knight_w: Texture2D,
    pawn_b: Texture2D,
    pawn_w: Texture2D,
    queen_b: Texture2D,
    queen_w: Texture2D,
    rook_b: Texture2D,
    rook_w: Texture2D,
}

fn get_asset_path(asset: &str) -> String {
    let path = Path::new(CRATE_PATH).join(ASSETS_PATH).join(asset);
    debug_assert!(path.exists(), "Asset not found: {:?}", path);
    path.to_str().unwrap().to_string()
}

impl Assets {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let bishop_b_path = get_asset_path(BISHOP_B);
        let bishop_w_path = get_asset_path(BISHOP_W);
        let king_b_path = get_asset_path(KING_B);
        let king_w_path = get_asset_path(KING_W);
        let knight_b_path = get_asset_path(KNIGHT_B);
        let knight_w_path = get_asset_path(KNIGHT_W);
        let pawn_b_path = get_asset_path(PAWN_B);
        let pawn_w_path = get_asset_path(PAWN_W);
        let queen_b_path = get_asset_path(QUEEN_B);
        let queen_w_path = get_asset_path(QUEEN_W);
        let rook_b_path = get_asset_path(ROOK_B);
        let rook_w_path = get_asset_path(ROOK_W);

        let bishop_b = rl
            .load_texture(thread, &bishop_b_path)
            .expect("Failed to load bishop_b texture");
        let bishop_w = rl
            .load_texture(thread, &bishop_w_path)
            .expect("Failed to load bishop_w texture");
        let king_b = rl
            .load_texture(thread, &king_b_path)
            .expect("Failed to load king_b texture");
        let king_w = rl
            .load_texture(thread, &king_w_path)
            .expect("Failed to load king_w texture");
        let knight_b = rl
            .load_texture(thread, &knight_b_path)
            .expect("Failed to load knight_b texture");
        let knight_w = rl
            .load_texture(thread, &knight_w_path)
            .expect("Failed to load knight_w texture");
        let pawn_b = rl
            .load_texture(thread, &pawn_b_path)
            .expect("Failed to load pawn_b texture");
        let pawn_w = rl
            .load_texture(thread, &pawn_w_path)
            .expect("Failed to load pawn_w texture");
        let queen_b = rl
            .load_texture(thread, &queen_b_path)
            .expect("Failed to load queen_b texture");
        let queen_w = rl
            .load_texture(thread, &queen_w_path)
            .expect("Failed to load queen_w texture");
        let rook_b = rl
            .load_texture(thread, &rook_b_path)
            .expect("Failed to load rook_b texture");
        let rook_w = rl
            .load_texture(thread, &rook_w_path)
            .expect("Failed to load rook_w texture");
        Self {
            bishop_b,
            bishop_w,
            king_b,
            king_w,
            knight_b,
            knight_w,
            pawn_b,
            pawn_w,
            queen_b,
            queen_w,
            rook_b,
            rook_w,
        }
    }
}

struct SelectedPiece {
    piece: Piece,
    game_index: usize,
    square_x: i32,
    square_y: i32,
    x: i32,
    y: i32,
}

fn draw_board(d: &mut RaylibDrawHandle) {
    let black_color = Color::from_hex("999999").unwrap();
    let mut white = false;
    for i in 0..BOARD_SIZE as i32 {
        for j in 0..BOARD_SIZE as i32 {
            let x = i * RECT_SIZE;
            let y = j * RECT_SIZE;
            let color = if white { Color::WHITE } else { black_color };
            d.draw_rectangle(x, y, RECT_SIZE, RECT_SIZE, color);
            white = !white;
        }
        white = !white;
    }
}

#[allow(dead_code)]
fn play_attacking_king(game: &mut Game) -> Option<UserOutput> {
    let possible_moves = game.get_all_currently_valid_moves();
    if possible_moves.is_empty() {
        panic!(
            "Something went wrong. No possible moves found. Function was probably called after check mate or stale mate."
        );
    }

    let move_to_play = possible_moves
        .iter()
        .find(|mv| {
            let mut game = game.clone();
            match game.process_input(&UserInput::Move(mv.from, mv.to)) {
                Some(UserOutput::CheckMate) => true,
                _ => game.check(game.turn.invert()),
            }
        })
        .unwrap_or_else(
            || match possible_moves.iter().find(|mv| mv.captured_piece.is_some()) {
                Some(mv) => mv,
                None => {
                    let rng = &mut rand::thread_rng();
                    let random_index = rng.gen_range(0..possible_moves.len());
                    &possible_moves[random_index]
                }
            },
        );

    game.process_input(&UserInput::Move(move_to_play.from, move_to_play.to))
}

#[allow(dead_code)]
fn play_randomly_aggressive(game: &mut Game) -> Option<UserOutput> {
    let possible_moves = game.get_all_currently_valid_moves();
    if possible_moves.is_empty() {
        panic!(
            "Something went wrong. No possible moves found. Function was probably called after check mate or stale mate."
        );
    }
    let move_to_play = match possible_moves.iter().find(|mv| mv.captured_piece.is_some()) {
        Some(mv) => mv,
        None => {
            let rng = &mut rand::thread_rng();
            let random_index = rng.gen_range(0..possible_moves.len());
            &possible_moves[random_index]
        }
    };

    game.process_input(&UserInput::Move(move_to_play.from, move_to_play.to))
}

#[inline]
const fn to_game_index(i: usize, j: usize) -> usize {
    TOTAL_SQUARES - 1 - i - j * BOARD_SIZE
}

#[inline]
const fn coord_to_game_index(x: i32, y: i32) -> usize {
    let i = x / RECT_SIZE;
    let j = y / RECT_SIZE;
    to_game_index(i as usize, j as usize)
}

const fn game_index_to_coord(index: usize) -> (i32, i32) {
    let index = TOTAL_SQUARES - 1 - index;
    let i = index % BOARD_SIZE;
    let j = index / BOARD_SIZE;
    (i as i32 * RECT_SIZE, j as i32 * RECT_SIZE)
}

fn draw_pieces(
    game: &Game,
    assets: &Assets,
    selected_piece: Option<&SelectedPiece>,
    d: &mut RaylibDrawHandle,
) {
    if let Some(selected_piece) = selected_piece {
        let pos = selected_piece
            .game_index
            .try_into()
            .expect("Invalid game index");
        let possible_moves = game.get_valid_moves(pos);
        for mv in possible_moves {
            let (x, y) = game_index_to_coord(mv.to.as_index());
            let color = if mv.captured_piece.is_some() {
                Color::from_hex("FF0000").unwrap()
            } else {
                Color::from_hex("00FF00").unwrap()
            };
            d.draw_rectangle(x, y, RECT_SIZE, RECT_SIZE, color.alpha(0.25));
        }
    }

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let game_index = to_game_index(i, j);
            let piece = game.board[game_index];
            if let Some(piece) = piece {
                let texture = match piece.color {
                    ChessColor::White => match piece.piece_type {
                        PieceType::Pawn => &assets.pawn_w,
                        PieceType::Rook => &assets.rook_w,
                        PieceType::Knight => &assets.knight_w,
                        PieceType::Bishop => &assets.bishop_w,
                        PieceType::Queen => &assets.queen_w,
                        PieceType::King => &assets.king_w,
                    },
                    ChessColor::Black => match piece.piece_type {
                        PieceType::Pawn => &assets.pawn_b,
                        PieceType::Rook => &assets.rook_b,
                        PieceType::Knight => &assets.knight_b,
                        PieceType::Bishop => &assets.bishop_b,
                        PieceType::Queen => &assets.queen_b,
                        PieceType::King => &assets.king_b,
                    },
                };
                let mut x = i as i32 * RECT_SIZE + RECT_SIZE / 2 - texture.width / 2;
                let mut y = j as i32 * RECT_SIZE + RECT_SIZE / 2 - texture.height / 2;
                if let Some(selected_piece) = selected_piece {
                    if selected_piece.piece == piece && selected_piece.game_index == game_index {
                        x = selected_piece.x - selected_piece.square_x;
                        y = selected_piece.y - selected_piece.square_y;
                    }
                }
                d.draw_texture(texture, x, y, Color::WHITE);
            }
        }
    }
}

fn draw(
    game: &Game,
    assets: &Assets,
    user_output: Option<&UserOutput>,
    selected_piece: Option<&SelectedPiece>,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) {
    let mut text = "";
    if let Some(user_output) = user_output {
        text = match user_output {
            UserOutput::CheckMate => "Checkmate!",
            UserOutput::StaleMate => "Stalemate!",
            UserOutput::Draw => "Draw!",
            UserOutput::InvalidMove => "Invalid move!",
            UserOutput::Promotion(_) => "Promotion!",
        };
    };
    let font_size = 60;
    let text_x = WINDOW_SIZE / 2 - rl.measure_text(text, font_size) / 2;
    let text_y = WINDOW_SIZE / 2 - font_size - font_size / 2;
    let text2 = "Press R to restart";
    let text2_x = WINDOW_SIZE / 2 - rl.measure_text(text2, font_size) / 2;
    let text2_y = WINDOW_SIZE / 2 + font_size - font_size / 2;

    /* ******* BEGIN DRAWING ******* */
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);
    draw_board(&mut d);
    draw_pieces(game, assets, selected_piece, &mut d);
    if !text.is_empty() {
        d.draw_text(text, text_x, text_y, font_size, Color::RED);
        d.draw_text(text2, text2_x, text2_y, font_size, Color::RED);
    }
}

fn update_game(
    game: &mut Game,
    selected_piece: &mut Option<SelectedPiece>,
    rl: &mut RaylibHandle,
) -> Option<UserOutput> {
    let user_output = if game.turn == ChessColor::White {
        update_selected_piece(game, selected_piece, rl)
    } else {
        play_attacking_king(game)
    };
    user_output
}

fn update_selected_piece(
    game: &mut Game,
    selected_piece: &mut Option<SelectedPiece>,
    rl: &mut RaylibHandle,
) -> Option<UserOutput> {
    let mut user_output = None;
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let mouse_pos = rl.get_mouse_position();
        let x = mouse_pos.x as i32;
        let y = mouse_pos.y as i32;
        if let Some(selected_piece) = selected_piece {
            selected_piece.x = x;
            selected_piece.y = y;
        } else {
            let game_index = coord_to_game_index(x, y);
            let square_x = x % RECT_SIZE;
            let square_y = y % RECT_SIZE;
            if let Some(piece) = game.board[game_index] {
                *selected_piece = Some(SelectedPiece {
                    piece,
                    game_index,
                    square_x,
                    square_y,
                    x,
                    y,
                });
            }
        }
    } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        && selected_piece.is_some()
    {
        let mouse_pos = rl.get_mouse_position();
        let x = mouse_pos.x as i32;
        let y = mouse_pos.y as i32;
        if let Ok(to) = coord_to_game_index(x, y).try_into() {
            let game_index = selected_piece.as_ref().unwrap().game_index;
            let from = game_index.try_into().expect("Invalid game index");
            let user_input = UserInput::Move(from, to);
            match game.process_input(&user_input) {
                Some(UserOutput::InvalidMove) => {
                    println!("Invalid move");
                }
                o => {
                    user_output = o;
                }
            }
        }
        *selected_piece = None;
    } else {
        *selected_piece = None;
    }
    user_output
}

fn main() {
    let available_cores = available_parallelism().unwrap().get();
    let used_cores = available_cores / 2;
    println!("{} cores available. Using {}.", available_cores, used_cores);
    rayon::ThreadPoolBuilder::new()
        .num_threads(used_cores)
        .build_global()
        .unwrap();

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title(TITLE)
        .msaa_4x() // anti-aliasing
        .build();
    let assets = Assets::new(&mut rl, &thread);

    let mut game = Game::new();

    rl.set_target_fps(60);
    rl.show_cursor();
    let mut finished = false;
    let mut user_output = None;
    let mut selected_piece = None;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            game = Game::new();
            finished = false;
            user_output = None;
        }
        if !finished {
            user_output = update_game(&mut game, &mut selected_piece, &mut rl);
            if user_output.is_some() {
                finished = true;
            }
        }
        draw(
            &game,
            &assets,
            user_output.as_ref(),
            selected_piece.as_ref(),
            &mut rl,
            &thread,
        );
    }
}
