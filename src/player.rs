use crate::board::Board;
use rand::{seq::SliceRandom, thread_rng};
use std::io;

pub struct Player {
    // 0 = human, 1 = random, 2 = randosmart, 3 = minimax
    player_type: i8,
    // 1 = red "X" (first), 2 = yellow "O" (second)
}

pub fn new_player(player_type: i8) -> Player {
    Player { player_type }
}

pub fn get_move(player: &Player, board: &mut Board) -> usize {
    match player.player_type {
        0 => get_human_move(),
        1 => random_move(board),
        // 2 => rando_smart_move(board),
        // 3 => minimax_move(board),
        _ => 0,
    }
}

// pub fn minimax_move(board: &mut Board) -> usize {}

// pub fn rando_smart_move(board: &mut Board) -> usize {}

pub fn random_move(board: &mut Board) -> usize {
    let lowest = board.lowest_empty.clone();
    // Remove the columns that are full with their index
    let empty: Vec<usize> = lowest
        // Gets iterator of lowest_empty
        .iter()
        // Enumerates the iterator
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
    return player_move_result - 1;
}
