use lazy_static::lazy_static;
use regex::Regex;
use rusty_chess_core::game::{Game, Piece, PieceType, Position, UserInput, UserOutput};
use std::io;
use std::io::BufRead;
use std::process::exit;

fn parse_input_move(std_input: &str) -> Result<UserInput, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?:\s*)?([a-zA-Z])(?:\s*)?(\d)(?:\s*)?(?:-|->)?(?:\s*)?([a-zA-Z])(?:\s*)?(\d)(?:\s*)?"
        )
        .unwrap();
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
            let from = Position(
                cap[1].chars().next().unwrap(),
                cap[2].chars().next().unwrap(),
            );
            let to = Position(
                cap[3].chars().next().unwrap(),
                cap[4].chars().next().unwrap(),
            );
            Ok(UserInput::Move(from, to))
        }
    }
}

fn headless_chess() {
    println!("Hello to rusty chess. Let's start a game:\n");
    let mut game = Game::new();
    let stdin = io::stdin();
    let mut previous_loop_turn = game.turn.invert();
    loop {
        if previous_loop_turn != game.turn {
            println!("{game}");
            println!(
                "{:?}'s turn. Please input a move (e.g. \"e2e4\" moves piece from e2 to e4)",
                game.turn
            );
        }
        previous_loop_turn = game.turn;
        let input_move = stdin.lock().lines().next().unwrap().unwrap();
        match parse_input_move(&input_move) {
            Err(e) => println!("{e}"),
            Ok(UserInput::Move(from, to)) => {
                let user_output = game.process_input(&UserInput::Move(from, to));
                if user_output.is_some() {
                    match user_output.unwrap() {
                        UserOutput::InvalidMove => {
                            println!("Not a valid move please repeat a move.");
                        }
                        UserOutput::Draw => {
                            println!("{game}");
                            println!("It is a draw!");
                            exit(0)
                        }
                        UserOutput::CheckMate => {
                            println!("{game}");
                            println!("{:?} has won!", game.turn.invert());
                            exit(0)
                        }
                        UserOutput::StaleMate => {
                            println!("{game}");
                            println!("It is a draw stalemate!");
                            exit(0)
                        }
                        UserOutput::Promotion(pos) => {
                            println!("{game}");
                            println!("To what piece do you want to promote your pawn (Queen, Rook, Knight, Bishop)?");
                            let promotion_str = stdin.lock().lines().next().unwrap().unwrap();
                            let color = game.turn;
                            if promotion_str.contains("Queen") || promotion_str.contains("queen") {
                                game.process_input(&UserInput::Promotion(
                                    Piece::new(PieceType::Queen, color),
                                    pos,
                                ));
                            } else if promotion_str.contains("Rook")
                                || promotion_str.contains("rook")
                            {
                                game.process_input(&UserInput::Promotion(
                                    Piece::new(PieceType::Rook, color),
                                    pos,
                                ));
                            } else if promotion_str.contains("Knight")
                                || promotion_str.contains("knight")
                            {
                                game.process_input(&UserInput::Promotion(
                                    Piece::new(PieceType::Knight, color),
                                    pos,
                                ));
                            } else if promotion_str.contains("Bishop")
                                || promotion_str.contains("Bishop")
                            {
                                game.process_input(&UserInput::Promotion(
                                    Piece::new(PieceType::Bishop, color),
                                    pos,
                                ));
                            } else {
                                println!("Invalid choice. Please choose between Queen, Rook, Bishop, Knight.");
                                continue;
                            }
                        }
                    }
                }
            }
            Ok(UserInput::Resign) => {
                println!("{:?} resigns!", game.turn);
                exit(0)
            }
            Ok(UserInput::Draw) => {
                println!(
                    "{:?} offers a draw does {:?} accept it? [y/N]",
                    game.turn,
                    game.turn.invert()
                );
                let input_move = stdin.lock().lines().next().unwrap().unwrap();
                if input_move.contains('y') {
                    println!("It is a draw!");
                    exit(0)
                }
                println!("Draw has been refused!");
            }
            Ok(UserInput::Promotion(_, _)) => {
                unreachable!("Should not be an output of parsing.")
            }
        }
    }
}

fn main() {
    headless_chess();
}
