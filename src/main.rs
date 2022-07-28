use std::time::Instant;

const GRID_SIZE: u8 = 9;
const GRID_BLOCK: u8 = 3;

type Board = [[u8; 9]; 9];

fn main() {
    let mut board: Board = [
        [7, 0, 2, 0, 5, 0, 6, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 0, 0],
        [1, 0, 0, 0, 0, 9, 5, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 9, 0],
        [0, 4, 3, 0, 0, 0, 7, 5, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 8],
        [0, 0, 9, 7, 0, 0, 0, 0, 5],
        [0, 0, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 7, 0, 4, 0, 2, 0, 3],
    ];

    let time = Instant::now();

    println!("\n    Original board:\n");
    print_board(&board);

    if solve_board(&mut board) {
        println!("\n    Solved board:\n");
        print_board(&board);
    } else {
        println!("\n  No solution found.");
    }

    println!("\nTime taken: {}ms", time.elapsed().as_millis());
}

fn is_number_in_row(board: &mut Board, number: u8, row: u8) -> bool {
    for i in 0..GRID_SIZE {
        if board[row as usize][i as usize] == number {
            return true;
        }
    }
    false
}

fn is_number_in_column(board: &mut Board, number: u8, column: u8) -> bool {
    for i in 0..GRID_SIZE {
        if board[i as usize][column as usize] == number {
            return true;
        }
    }
    false
}

fn is_number_in_box(board: &mut Board, number: u8, box_row: u8, box_column: u8) -> bool {
    for i in 0..GRID_BLOCK {
        for j in 0..GRID_BLOCK {
            if board[(box_row + i) as usize][(box_column + j) as usize] == number {
                return true;
            }
        }
    }
    false
}

fn is_valid_place(board: &mut Board, number: u8, row: u8, column: u8) -> bool {
    !is_number_in_row(board, number, row)
        && !is_number_in_column(board, number, column)
        && !is_number_in_box(
            board,
            number,
            row - row % GRID_BLOCK,
            column - column % GRID_BLOCK,
        )
}

fn solve_board(board: &mut Board) -> bool {
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if board[i as usize][j as usize] == 0 {
                for k in 1..=GRID_SIZE {
                    if is_valid_place(board, k, i, j) {
                        board[i as usize][j as usize] = k;
                        if solve_board(board) {
                            return true;
                        }
                        board[i as usize][j as usize] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn print_board(board: &Board) {
    for i in 0..GRID_SIZE {
        if i % GRID_BLOCK == 0 && i != 0 {
            println!("-----------------------");
        }
        for j in 0..GRID_SIZE {
            if j % GRID_BLOCK == 0 && j != 0 {
                print!(" | ");
            }
            print!("{} ", board[i as usize][j as usize]);
        }
        println!();
    }
}
