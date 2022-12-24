use crate::board;
use crate::board::Board;
use rand::{seq::SliceRandom, thread_rng};
use std::io;

#[derive(Copy, Clone)]
pub struct Player {
    // 0 = human, 1 = random, 2 = randosmart, 3 = minimax
    player_type: i8,
    // 1 = red "X" (first), 2 = yellow "O" (second)
    player_piece: usize,
}

pub fn new_player(player_type: i8, player_piece: usize) -> Player {
    Player {
        player_type,
        player_piece,
    }
}

pub fn get_move(player: &Player, board: &mut Board) -> usize {
    match player.player_type {
        0 => get_human_move(),
        1 => random_move(board),
        2 => randosmart_move(&player, board),
        3 => minimax_move(&player, board),
        _ => 0,
    }
}

pub fn minimax_move(player: &Player, board: &mut Board) -> usize {
    let mut rng = thread_rng();
    let mut next_turn_wins = Vec::new();
    let mut t_board = board::clone_board(board);
    let depth = 3;

    // finding any immediate wins
    for i in 0..7 {
        if board::add_piece(&mut t_board, i, player.player_piece) {
            if board::game_over_check(&mut t_board) {
                next_turn_wins.push(i);
            }
            board::undo_move(&mut t_board);
        }
    }
    // randomly select a move that wins next turn
    if next_turn_wins.len() > 0 {
        println!("Found i-winning move: {:?}", next_turn_wins);
        return *next_turn_wins.choose(&mut rng).unwrap();
    }

    let opponent_color = if player.player_piece == 1 { 2 } else { 1 };

    // Finding minimax values for each column
    let empty = board::get_empty_columns(board);
    let mut column_values: Vec<i32> = Vec::new();
    for column in empty {
        if board::add_piece(&mut t_board, column, player.player_piece) {
            column_values.push(minimax(&mut t_board, depth, *player, opponent_color));
            board::undo_move(&mut t_board);
        } else {
            column_values.push(-f64::INFINITY as i32);
        }
    }

    println!("VALUES: {:?}", column_values);

    let empty = board::get_empty_columns(board);
    println!("COLUMN: {:?}", empty);
    // Return max of the column_values
    let mut max = column_values[0];
    let mut max_index = 0;
    for i in 0..empty.len() {
        if column_values[i] > max {
            max = column_values[i];
            max_index = i;
        }
    }
    println!("Max value: {}", max);

    return empty[max_index];
}

pub fn minimax(board: &mut Board, depth: i32, player: Player, color: usize) -> i32 {
    let p_color = player.player_piece;
    let o_color = if p_color == 1 { 2 } else { 1 };

    // If 0 depth, return 0, no information was gained
    if depth == 0 {
        return 0;
    }

    // If no more empty columns, return 0, it's a tie
    let empty = board::get_empty_columns(board);
    if empty.len() == 0 {
        return 0;
    }

    // Keeping track of best scores
    let mut best_score: f64;
    if color == player.player_piece {
        // If it's the players turn, we want to maximize the score
        // So we start with the lowest possible score
        best_score = -f64::INFINITY;
    } else {
        // Opposite for opponent
        best_score = f64::INFINITY;
    }

    let win_score: i32 = 100 * depth;
    let lose_score: i32 = -100 * depth;

    let mut t_board = board::clone_board(board);
    // For each empty column
    for column in empty.iter() {
        // If we can add a piece to the column
        let piece_place = board::add_piece(&mut t_board, *column, color);
        let win = board::game_over_check(board);
        // If we win, return win score, depending on player color and depth
        if win {
            board::undo_move(&mut t_board);
            if color == p_color {
                println!("WON {} {}", win_score, column);
                return win_score;
            } else {
                println!("LOST {} {}", lose_score, column);
                return lose_score;
            }
        }
        // If there is no immediate win, we need to check the next depth
        if color == p_color {
            let score = minimax(&mut t_board, depth - 1, player, o_color);
            best_score = best_score.max(score as f64);
        } else {
            let score = minimax(&mut t_board, depth - 1, player, p_color);
            best_score = best_score.min(score as f64);
        }
        // Undo move
        board::undo_move(&mut t_board);
    }

    return best_score as i32;
}

//  Looks for an immediate win, if it can't find one, it looks for an immediate loss,
// if it can't find one, it makes a random move
pub fn randosmart_move(player: &Player, board: &mut Board) -> usize {
    // Checks if randomsmart can win by placing a piece in some column
    for i in 0..6 {
        board::add_piece(board, i, player.player_piece);
        if board::game_over_check(board) {
            println!("Found a winning move: {}", i);
            return i;
        }
        board::undo_move(board);
    }

    // Clone board
    let mut temp_board = board::clone_board(board);
    for i in 0..6 {
        // Get opponent's piece and place it in column i
        let opponent_piece = if player.player_piece == 1 { 2 } else { 1 };
        board::add_piece(&mut temp_board, i, opponent_piece);

        // If opponent can win, place piece in column i to block win
        if board::game_over_check(&mut temp_board) {
            return i;
        } else {
            // Undo the move
            board::undo_move(&mut temp_board);
        }
    }

    // If there is no immediate win or loss, make a random move
    return random_move(board);
}

pub fn random_move(board: &mut Board) -> usize {
    let lowest = board.lowest_empty.clone();
    // Remove the columns that are full with their index
    let empty: Vec<usize> = lowest
        // Gets iterator of lowest_empty
        .iter()
        // Enumerates through the iterator
        .enumerate()
        // Filters out the columns that are = 0, so full
        .filter(|&(_, &x)| x != 0)
        // Maps the iterator to just the index
        .map(|(i, _)| i)
        // Collects the iterator into a vector
        .collect();

    let mut rng = thread_rng();
    let num = empty.choose(&mut rng);
    return *num.unwrap() as usize;
}

pub fn get_human_move() -> usize {
    //println!("Enter a column number (1-7): ");
    let mut player_move = String::new();
    let mut player_move_result: usize;
    loop {
        player_move.clear();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");
        let _player_move: u32 = match player_move.trim().parse::<usize>() {
            Ok(num) => {
                player_move_result = num as usize;
                if player_move_result > 0 && player_move_result < 8 {
                    break;
                } else {
                    //println!("Invalid move");
                    continue;
                }
            }
            Err(_) => {
                //println!("Invalid input");
                continue;
            }
        };
    }
    println!();
    return player_move_result - 1;
}
