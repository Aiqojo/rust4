// use rand::{random, seq::SliceRandom, thread_rng, Rng};
// use std::{cmp::min, io, os::windows::thread};
use std::env;
mod board;
mod player;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const TOTAL_GAMES: f32 = 1000000.0;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // set a timer to find out how many games per second the computer can play
    let start = std::time::Instant::now();
    let mut game_number = 1;
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    let mut ties = 0;

    for _ in 0..TOTAL_GAMES as usize {
        let winner: usize;

        // 0 = human, 1 = random, 2 = randosmart, 3 = minimax
        winner = game_handler(2, 2, false);

        match winner {
            1 => player1_wins += 1,
            2 => player2_wins += 1,
            3 => ties += 1,
            _ => println!("Error: winner is not 1, 2, or 3"),
        }

        game_number += 1;
        if game_number % 10000 == 0 {
            println!("{} games played", game_number);
        }
    }

    let end = std::time::Instant::now();
    let elapsed = end - start;
    println!("{} milisecond elapsed", elapsed.as_millis());
    println!(
        "{} games per milisecond",
        TOTAL_GAMES / elapsed.as_millis() as f32
    );
    println!("{} games per second", TOTAL_GAMES / elapsed.as_secs_f32());
    println!("{} games won by player 1", player1_wins);
    println!("{} games won by player 2", player2_wins);
    println!("{} games tied", ties);
}

fn game_handler(p1: usize, p2: usize, print: bool) -> usize {
    let mut board: board::Board = board::new_board(WIDTH, HEIGHT);
    let winner: usize;

    if print {
        board::print_board(&mut board);
    }

    // player types: 1 = player, 2 = random, 3 = randosmart
    winner = game_loop(&mut board, print, p1 as i8, p2 as i8);

    if print {
        println!("Winner: {}", winner);
    }

    if print {
        board::print_board(&mut board);
    }
    return winner;
}

fn game_loop(board: &mut board::Board, print: bool, p1_type: i8, p2_type: i8) -> usize {
    // Tracks turn so we know which player's turn it is
    let mut turn: i32 = 1;
    // Tracks winner
    let mut winner: usize;
    // Creates a player object for each player
    // player_type: 0 = human, 1 = random, 2 = randosmart, 3 = minimax
    let player1 = player::new_player(p1_type, 1);
    let player2 = player::new_player(p2_type, 2);
    // Tracks most recent move
    let mut player_move: usize;

    // Game loop that only breaks upon tie or win
    loop {
        // Gets the player piece based on turn
        let player_piece: usize = board::get_player_piece(turn);
        // If turn is odd, it's player 1's turn
        if player_piece == 1 {
            player_move = player::get_move(&player1, board);
        } else {
            player_move = player::get_move(&player2, board);
        }

        // Makes the move and checks if it was valid
        let player_move_result: bool = board::make_move(player_move, player_piece, board);
        // If it was valid, we increment the turn and print the board if print is true
        if player_move_result {
            turn = turn + 1;
            if print {
                println!("Turn: {turn}");
                println!(
                    "Player {} moved to column {}",
                    player_piece,
                    player_move + 1
                );
                board::print_board(board);
            }
        } else {
            if print {
                println!("Invalid move");
            }
        }
        // Checks if the game is over
        winner = is_game_over(board, turn, player_move);
        // 0 = no winner yet, 1 = player 1 wins, 2 = player 2 wins, 3 = tie
        if winner == 0 {
            continue;
        } else {
            // println!("Winner: {}", winner);
            break;
        }
    }
    return winner;
}

//0 is not game over, 1 red won, 2 yellow won, 3 tie
fn is_game_over(board: &mut board::Board, turn: i32, column: usize) -> usize {
    let mut game_over_status: usize = 0;
    if turn > 42 {
        game_over_status = 3;
        //println!("TIE");
        return game_over_status;
    } else {
        let win: bool = board::game_over_check(board, column);
        if win {
            // Because turn is incremented before this function is called, we need to subtract 1
            if (turn - 1) % 2 == 1 {
                game_over_status = 1;
                //println!("RED WON");
            } else {
                game_over_status = 2;
                //println!("YELLOW WON");
            }
        }
    }
    return game_over_status;
}
