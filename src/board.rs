// use rand::{random, seq::SliceRandom, thread_rng, Rng};
use std::cmp::min;

pub struct Board {
    width: usize,
    // height: usize,
    grid: Vec<Vec<usize>>,
    pub lowest_empty: Vec<usize>,
}

pub fn new_board(width: usize, height: usize) -> Board {
    Board {
        width,
        // height,
        grid: vec![vec![0; width as usize]; height as usize],
        lowest_empty: vec![height; width],
    }
}

pub fn clone_board(board: &Board) -> Board {
    Board {
        width: board.width,
        // height: board.height,
        grid: board.grid.clone(),
        lowest_empty: board.lowest_empty.clone(),
    }
}

pub fn make_move(player_move: usize, player_piece: usize, board: &mut Board) -> bool {
    // Checks if player_move in bounds
    if player_move > board.width - 1 {
        return false;
    }
    return add_piece(board, player_move, player_piece);
}

pub fn add_piece(board: &mut Board, col: usize, piece: usize) -> bool {
    // If column is out of boudns return false
    if col > board.width - 1 {
        return false;
    }
    // If column is full return false
    if board.lowest_empty[col] == 0 {
        return false;
    }
    // Add piece to board
    board.grid[board.lowest_empty[col] - 1][col] = piece;
    // Increment lowest empty
    board.lowest_empty[col] -= 1;
    return true;
}

//game starts on turn 1, player 1 goes first, so have to subtract 1
pub fn get_player_piece(turn: i32) -> usize {
    if (turn - 1) % 2 == 0 {
        return 1;
    } else {
        return 2;
    }
}

pub fn print_board(board: &mut Board) {
    // 0 = empty, 1 = red/X, 2 = yellow/O
    for i in 0..6 {
        for j in 0..7 {
            match board.grid[i][j] {
                0 => print!("  ."),
                1 => print!("  X"),
                2 => print!("  O"),
                _ => print!("  ?"),
            }
        }
        println!();
    }
    println!();
}

pub fn game_over_check(board: &mut Board, column: usize) -> bool {
    // keep track of how many pieces are in a row
    let mut win_counter: usize = 0;
    //gets the lowest row of a piece in the column that was selected
    // 6 = empty column, 0 = full column
    let row: isize = board.lowest_empty[column] as isize;
    // If column is empty return false, there can be no possible win
    if row == 6 {
        return false;
    }

    //grabs the color (either 1 or 2) of the piece that was selected
    // print row and column
    // println!("row: {}, column: {}", row, column);
    let color: usize = board.grid[row as usize][column];

    //println!("Game check: row: {row}, column: {column}, color: {color}");

    // ------------------
    // ----HORIZONTAL----
    // ------------------

    // Just keep going left in this row until you get win_counter to 3
    // or you run into the opposite color
    // keeping track of the last column there was a correct color piece,
    // you go back the other way and check if there are enough to the left
    // to make a 4 in a row
    //start one offset to the right because we know the first one is correct
    for i in (column + 1)..board.grid[row as usize].len() as usize {
        if board.grid[row as usize][i as usize] == color {
            //println!("HORIZONTAL: {row}, {i}, {color}");
            win_counter = win_counter + 1;
        //if its not the same color, break we move to looking to the left
        } else {
            break;
        }

        if win_counter == 3 {
            return true;
        }
        //println!("HORIZONTAL: win_counter: {win_counter}");
    }
    //look to the right starting from the original column
    if column > 0 {
        for i in (0..column).rev() {
            if board.grid[row as usize][i as usize] == color {
                //println!("HORIZONTAL REVERSE: {row}, {i}, {color}");
                win_counter = win_counter + 1;
            } else {
                break;
            }

            if win_counter == 3 {
                return true;
            }
        }
        //println!("HORIZONTAL: win_counter: {win_counter}");
    }

    // ----------------
    // ----VERTICAL----
    // ----------------

    win_counter = 0;

    // You cant win vertical if there isn't already at least 3 pieces underneath
    // so we dont check those
    if row < 3 {
        let mut row_vertical: isize = row + 1;
        // //println!("VERTICAL: row_vertical: {row_vertical}");

        while row_vertical < board.grid.len() as isize {
            if board.grid[row_vertical as usize][column as usize] == color {
                // //println!("VERTICAL PIECE: {row_vertical}, {column}, {color}");
                win_counter = win_counter + 1;
            } else {
                break;
            }
            row_vertical = row_vertical + 1;

            if win_counter == 3 {
                return true;
            }
        }
    }

    // ----------------
    // ----DIAGONAL----
    // ----------------

    // Diagonal down and to the right
    // Capital X is current piece placed
    // X
    //   x
    //     x
    //       x

    win_counter = 0;

    // This looks for the furthest you can go eiter down or to the right
    // and takes the minimum so we dont go out of bounds
    let furthest_distance_dr = min(
        board.grid.len() as isize - row,
        board.grid[0].len() as isize - column as isize,
    );
    // let furthest_distance_dr = min(HEIGHT - row, WIDTH - column);
    //println!("furthest_distance: {furthest_distance_dr}");
    for i in 1..furthest_distance_dr {
        if board.grid[(row + i) as usize][(column + i as usize) as usize] == color {
            //println!("DIAGONAL: {row}, {column}, {color}");
            win_counter = win_counter + 1;
        } else {
            break;
        }
        if win_counter == 3 {
            return true;
        }
    }

    // Diagonal up and to the left
    // Capital X is current piece placed
    //   x
    //     x
    //       x
    //         X
    let furthest_distance_ul = min(row, column as isize);
    //println!("furthest_distance: {furthest_distance_ul}");
    for i in 1..furthest_distance_ul {
        if board.grid[(row - i) as usize][(column - i as usize) as usize] == color {
            //println!("DIAGONAL: {row}, {column}, {color}");
            win_counter = win_counter + 1;
        } else {
            break;
        }
        if win_counter == 3 {
            return true;
        }
    }

    // Diagonal down and to the left
    // Capital X is current piece placed
    //       x
    //     x
    //   x
    // X

    win_counter = 0;
    let furthest_distance_dl = min(board.grid.len() as usize - row as usize - 1, column) + 1;
    //println!("furthest_distance: {furthest_distance_dl}");
    for i in 1..furthest_distance_dl {
        if board.grid[(row + i as isize) as usize][(column - i) as usize] == color {
            //println!("DIAGONAL: {row}, {column}, {color}");
            win_counter = win_counter + 1;
        } else {
            break;
        }
        if win_counter == 3 {
            return true;
        }
    }

    // Diagonal up and to the right
    // Capital X is current piece placed
    //       x
    //     x
    //   x
    // X

    let furthest_distance_ur = min(row, (board.grid[0].len() - column - 1) as isize) + 1;
    //println!("furthest_distance: {furthest_distance_ur}");
    for i in 1..furthest_distance_ur {
        if board.grid[(row - i) as usize][(column + i as usize) as usize] == color {
            //println!("DIAGONAL: {row}, {column}, {color}");
            win_counter = win_counter + 1;
        } else {
            break;
        }
        if win_counter == 3 {
            return true;
        }
    }

    return false;
}

// Below is replaced by board.lowest_empty
// pub fn update_valid_rows(board: &mut Board) {
//     let mut valid_rows: Vec<usize> = Vec::new();
//     for i in 0..=6 {
//         if lowest_row(board, i) != -1 {
//             valid_rows.push(i);
//         }
//     }
//     board.lowest_empty = valid_rows;
// }

// pub fn lowest_row(board: &mut Board, col: usize) -> isize {
//     for i in (0..6).rev() {
//         if board.grid[i][col as usize] == 0 {
//             return i as isize;
//         }
//     }
//     return -1;
// }
