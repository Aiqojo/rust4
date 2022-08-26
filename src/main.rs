use std::{cmp::min, io};

const WIDTH: u8 = 7;
const HEIGHT: u8 = 6;

fn main() {
    let mut board: [[u8; WIDTH as usize]; HEIGHT as usize] = [[0; WIDTH as usize]; HEIGHT as usize];
    let winner: u8;
    winner = game_loop(&mut board);
    if winner == 1 {
        println!("Player 1 wins!");
    } else if winner == 2 {
        println!("Player 2 wins!");
    } else {
        println!("Draw!");
    }
    print_board(&board);
}

fn game_loop(board: &mut [[u8; 7]; 6]) -> u8 {
    let mut turn: i32 = 1;
    let mut winner: u8;

    loop {
        let player_move: u8 = get_player_move();
        let player_piece: u8 = get_player_piece(turn);
        let player_move_result: bool = make_move(player_move, player_piece, board);
        if player_move_result {
            turn = turn + 1;
            println!("Turn: {turn}");
            print_board(board);
        } else {
            println!("Invalid move");
        }
        winner = is_game_over(board, turn, player_move);
        if winner == 0 {
            continue;
        } else {
            break;
        }
    }
    return winner;
}

//0 is not game over, 1 red won, 2 yellow won, 3 tie
fn is_game_over(board: &[[u8; 7]; 6], turn: i32, column: u8) -> u8 {
    let mut game_over_status: u8 = 0;
    if turn >= 42 {
        game_over_status = 3;
        println!("TIE");
        return game_over_status;
    } else {
        let win: bool = game_over_check(board, column);
        if win {
            if turn % 2 == 0 {
                game_over_status = 1;
                println!("RED WON");
            } else {
                game_over_status = 2;
                println!("YELLOW WON");
            }
        }
    }
    return game_over_status;
}

fn make_move(player_move: u8, player_piece: u8, board: &mut [[u8; 7]; 6]) -> bool {
    return add_piece(board, player_move, player_piece);
}

//game starts on turn 1, player 1 goes first, so have to subtract 1
fn get_player_piece(turn: i32) -> u8 {
    if (turn - 1) % 2 == 0 {
        return 1;
    } else {
        return 2;
    }
}

fn get_player_move() -> u8 {
    println!("Enter a column number (1-7): ");
    let mut player_move = String::new();
    let player_move_result: u8;
    loop {
        player_move.clear();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");
        let _player_move: u32 = match player_move.trim().parse::<u8>() {
            Ok(num) => {
                player_move_result = num as u8;
                break;
            }
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
    }
    return player_move_result - 1;
}

fn print_board(board: &[[u8; 7]; 6]) {
    // 0 = empty, 1 = red/X, 2 = yellow/O
    for i in 0..6 {
        for j in 0..7 {
            match board[i][j] {
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

fn add_piece(board: &mut [[u8; 7]; 6], col: u8, piece: u8) -> bool {
    if col > 6 {
        return false;
    }
    for i in (0..6).rev() {
        if board[i][col as usize] == 0 {
            board[i][col as usize] = piece;
            return true;
        }
    }
    return false;
}

fn lowest_row(board: &[[u8; 7]; 6], col: u8) -> u8 {
    for i in (0..6).rev() {
        if board[i][col as usize] == 0 {
            return i as u8;
        }
    }
    return 0;
}

fn game_over_check(board: &[[u8; 7]; 6], column: u8) -> bool {
    // keep track of how many pieces are in a row
    let mut win_counter: u8 = 0;
    //gets the lowest row of a piece in the column that was selected
    // is one below the lowest 0, which is found by lowest_row()
    let row: u8 = lowest_row(board, column) + 1;
    //grabs the color (either 1 or 2) of the piece that was selected
    let color: u8 = board[row as usize][column as usize];

    //println!("GOOB row: {row}, column: {column}, color: {color}");

    // ------------------
    // ----HORIZONTAL----
    // ------------------

    // Just keep going left in this row until you get win_counter to 3
    // or you run into the opposite color
    // keeping track of the last column there was a correct color piece,
    // you go back the other way and check if there are enough to the left
    // to make a 4 in a row
    //start one offset to the right because we know the first one is correct
    for i in (column + 1)..(WIDTH as u8) {
        if board[row as usize][i as usize] == color {
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
            if board[row as usize][i as usize] == color {
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

    let mut row_vertical: u8 = min(row + 1, HEIGHT - 1);

    while row_vertical < HEIGHT {
        if board[row_vertical as usize][column as usize] == color {
            //println!("VERTICAL: {row_vertical}, {column}, {color}");
            win_counter = win_counter + 1;
        } else {
            break;
        }
        row_vertical = row_vertical + 1;

        if win_counter == 3 {
            return true;
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
    let furthest_distance_dr = min(HEIGHT - row, WIDTH - column);
    //println!("furthest_distance: {furthest_distance_dr}");
    for i in 1..furthest_distance_dr {
        if board[(row + i) as usize][(column + i) as usize] == color {
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
    win_counter = 0;
    let furthest_distance_ul = min(row, column);
    //println!("furthest_distance: {furthest_distance_ul}");
    for i in 1..furthest_distance_ul {
        if board[(row - i) as usize][(column - i) as usize] == color {
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
    let furthest_distance_dl = min(HEIGHT - row - 1, column) + 1;
    //println!("furthest_distance: {furthest_distance_dl}");
    for i in 1..furthest_distance_dl {
        if board[(row + i) as usize][(column - i) as usize] == color {
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

    win_counter = 0;
    let furthest_distance_ur = min(row, WIDTH - column - 1) + 1;
    //println!("furthest_distance: {furthest_distance_ur}");
    for i in 1..furthest_distance_ur {
        if board[(row - i) as usize][(column + i) as usize] == color {
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
