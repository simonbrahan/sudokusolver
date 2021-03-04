pub type Board = [Option<usize>; 81];

use std::collections::HashSet;
use std::convert::TryFrom;

pub fn solve(board: &Board) -> Option<Board> {
    let empty_cells = get_empty_cells(board);

    if empty_cells.is_empty() {
        return Some(*board);
    }

    let cells_options = get_cells_options(board, empty_cells);

    for (cell, options) in cells_options {
        if options.is_empty() {
            return None;
        }

        for option in options {
            let next_board = change_board(board, cell, option);

            if let Some(solved) = solve(&next_board) {
                return Some(solved);
            }
        }
    }

    None
}

pub fn parse(input: &str) -> Result<Board, &str> {
    let game_board: Vec<Option<usize>> = input
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>()
        .split(',')
        .map(|num| num.parse().ok())
        .collect();

    if game_board.len() != 81 {
        dbg!(game_board);
        return Err("Board must be 81 symbols in length");
    }

    if game_board
        .iter()
        .any(|maybe_num| maybe_num.unwrap_or(0) > 9)
    {
        return Err("Board may only contain symbols 1 to 9 or blank squares");
    }

    Ok(Board::try_from(game_board).unwrap())
}

pub fn print_board(board: &Board) {
    let board_symbols: Vec<usize> = board
        .iter()
        .map(|maybe_num| maybe_num.unwrap_or(0))
        .collect();

    let board_lines = board_symbols.chunks(9);

    for line in board_lines {
        println!(
            "{}",
            line.iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

fn get_empty_cells(board: &Board) -> Vec<usize> {
    let mut out = Vec::<usize>::new();

    for (idx, space) in board.iter().enumerate() {
        if space.is_none() {
            out.push(idx);
        }
    }

    out
}

fn get_cells_options(board: &Board, cells: Vec<usize>) -> Vec<(usize, Vec<usize>)> {
    let mut out = Vec::<(usize, Vec<usize>)>::new();

    for cell in cells {
        out.push((cell, get_cell_options(board, cell)));
    }

    // We want to see cells with no options (unsolvable board)
    // or only one option (easy move) first.
    out.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    out
}

fn get_cell_options(board: &Board, cell_idx: usize) -> Vec<usize> {
    let mut taken = HashSet::new();

    // Get symbols taken by column
    let column = cell_idx % 9;
    for cell in board.iter().skip(column).step_by(9) {
        if cell.is_some() {
            taken.insert(cell.unwrap());
        }
    }

    // Get symbols taken by row
    let row_start = cell_idx - column;
    let after_row_end = row_start + 9;
    for cell in board.iter().take(after_row_end).skip(row_start) {
        if cell.is_some() {
            taken.insert(cell.unwrap());
        }
    }

    // get symbols taken by square
    let offsets_in_square = [0, 1, 2, 9, 10, 11, 18, 19, 20];
    let square_start_column = column - column % 3;
    let square_start_cell_idx = cell_idx - cell_idx % 27 + square_start_column;
    for offset in &offsets_in_square {
        let cell = board[square_start_cell_idx + offset];
        if let Some(cell_symbol) = cell {
            taken.insert(cell_symbol);
        }
    }

    // get symbols that are not taken by any related cell
    let available = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut out = Vec::new();
    for candidate in &available {
        if !taken.contains(&candidate) {
            out.push(*candidate);
        }
    }

    out
}

pub fn change_board(board: &Board, cell: usize, new_val: usize) -> Board {
    let mut out = [None; 81];

    for (idx, existing_val) in board.iter().enumerate() {
        if idx == cell {
            out[idx] = Some(new_val);
        } else {
            out[idx] = *existing_val;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_easy() {
        let start_board = parse(
            "
           4,8,3,9, , ,6,5,7,
           9,6,7, ,4,5,8, , ,
            ,5, ,8,7,6,4, ,3,
           5,4,8, , , , ,7,6,
           7, ,9,5,6,4, ,3,8,
            ,3, ,7,9, , ,4,5,
           3,7, ,6,8,9,5, ,4,
           8, ,4, , , ,7,6,9,
           6,9,5,4, ,7,3,8,2
        ",
        )
        .unwrap();

        let solved_board = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9,6,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4,9,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3,7,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        assert_eq!(Some(solved_board), solve(&start_board));
    }

    #[test]
    fn cell_options_from_full_board() {
        let full_board = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9,6,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4,9,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3,7,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        assert_eq!(Vec::<usize>::new(), get_cell_options(&full_board, 0))
    }

    #[test]
    fn cell_options_from_partial_column() {
        let board_with_partial_column = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9,6,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4, ,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3,7,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        assert_eq!(vec![9], get_cell_options(&board_with_partial_column, 25))
    }

    #[test]
    fn cell_options_from_partial_row() {
        let board_with_partial_row = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9,6,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4,9,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3, ,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        assert_eq!(vec![7], get_cell_options(&board_with_partial_row, 55))
    }

    #[test]
    fn cell_options_from_partial_square() {
        let board_with_partial_row = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9, ,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4,9,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3,7,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        assert_eq!(vec![6], get_cell_options(&board_with_partial_row, 10))
    }

    #[test]
    fn parse_board_too_small() {
        assert_eq!(
            Err("Board must be 81 symbols in length"),
            parse(&",".repeat(79))
        );
    }

    #[test]
    fn parse_board_too_large() {
        assert_eq!(
            Err("Board must be 81 symbols in length"),
            parse(&",".repeat(81))
        );
    }

    #[test]
    fn parse_board_invalid_symbol() {
        assert_eq!(
            Err("Board may only contain symbols 1 to 9 or blank squares"),
            parse(&(",".repeat(80) + "10"))
        );
    }

    #[test]
    fn add_val_to_board() {
        let original_board = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9, ,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4,9,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3,7,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        let new_board = parse(
            "
           4,8,3,9,2,1,6,5,7,
           9,6,7,3,4,5,8,2,1,
           2,5,1,8,7,6,4,9,3,
           5,4,8,1,3,2,9,7,6,
           7,2,9,5,6,4,1,3,8,
           1,3,6,7,9,8,2,4,5,
           3,7,2,6,8,9,5,1,4,
           8,1,4,2,5,3,7,6,9,
           6,9,5,4,1,7,3,8,2
        ",
        )
        .unwrap();

        assert_eq!(new_board, change_board(&original_board, 10, 6))
    }
}
