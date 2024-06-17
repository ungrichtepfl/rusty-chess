use rand::Rng;
use raylib::prelude::*;
use rusty_chess_core::game::Color as ChessColor;
use rusty_chess_core::game::Game;
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

fn draw_pieces(game: &Game, assets: &Assets, d: &mut RaylibDrawHandle) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let piece = game.board[TOTAL_SQUARES - 1 - i - j * BOARD_SIZE];
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
                let x = i as i32 * RECT_SIZE + RECT_SIZE / 2 - texture.width / 2;
                let y = j as i32 * RECT_SIZE + RECT_SIZE / 2 - texture.height / 2;
                d.draw_texture(texture, x, y, Color::WHITE);
            }
        }
    }
}
fn draw(
    game: &Game,
    assets: &Assets,
    user_output: Option<&UserOutput>,
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

    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);
    draw_board(&mut d);
    draw_pieces(game, assets, &mut d);
    if !text.is_empty() {
        d.draw_text(text, text_x, text_y, font_size, Color::RED);
        d.draw_text(text2, text2_x, text2_y, font_size, Color::RED);
    }
}
fn update_game(game: &mut Game) -> Option<UserOutput> {
    let user_output = if game.turn == ChessColor::White {
        play_randomly_aggressive(game)
    } else {
        play_attacking_king(game)
    };
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
    let mut finished = false;
    let mut user_output = None;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            game = Game::new();
            finished = false;
            user_output = None;
        }
        if !finished {
            user_output = update_game(&mut game);
            if user_output.is_some() {
                finished = true;
            }
        }
        draw(&game, &assets, user_output.as_ref(), &mut rl, &thread);
    }
}
