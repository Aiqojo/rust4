// use rand::{random, seq::SliceRandom, thread_rng, Rng};
// use std::{cmp::min, io, os::windows::thread};
mod board;
mod player;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

fn main() {
    // set a timer to find out how many games per second the computer can play
    let start = std::time::Instant::now();
    let mut game_number = 1;
    for _ in 0..1000000 {
        // game types: 1 = player vs player, 2 = random game, 3 = player vs random
        game_handler(2, false);
        //println!("Game {} over", game_number);
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
        1000000.0 / elapsed.as_millis() as f64
    );
    println!("{} games per second", 1000000.0 / elapsed.as_secs_f64());
}

fn game_handler(game_type: usize, print: bool) {
    let mut board: board::Board = board::new_board(WIDTH, HEIGHT);
    let winner: usize;

    // game types: 1 = player vs player, 2 = random game, 3 = player vs random
    if game_type == 1 {
        winner = game_loop(&mut board, print, 0, 0);
    } else if game_type == 2 {
        winner = game_loop(&mut board, print, 1, 1);
    } else {
        panic!("Invalid game type");
    }

    if print {
        println!("Winner: {}", winner);
    }

    if print {
        board::print_board(&mut board);
    }
}

fn game_loop(board: &mut board::Board, print: bool, p1_type: i8, p2_type: i8) -> usize {
    // Tracks turn so we know which player's turn it is
    let mut turn: i32 = 1;
    // Tracks winner
    let mut winner: usize;
    // Creates a player object for each player
    // player_type: 0 = human, 1 = random, 2 = randosmart, 3 = minimax
    let player1 = player::new_player(p1_type);
    let player2 = player::new_player(p2_type);
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
