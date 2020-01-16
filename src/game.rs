pub type Board = [Option<usize>];

use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve(board: &Board) { //}-> Option<Board> {
    let mut candidate_boards: VecDeque<&Board> = VecDeque::new();
    candidate_boards.push_back(&board);

    'boards: while !candidate_boards.is_empty() {
        let candidate_board = candidate_boards.pop_front().unwrap();
        let empty_cells = get_empty_cells(&board);

        if empty_cells.is_empty() {
            print_board(candidate_board);
            //return Some(candidate_board);
        }

        for empty_cell_idx in empty_cells {
            let cell_options = get_cell_options(&candidate_board, empty_cell_idx);

            if cell_options.is_empty() {
                continue 'boards;
            }

            for cell_option in cell_options {
                let mut new_candidate_board = candidate_board.to_vec();
                new_candidate_board[empty_cell_idx] = Some(cell_option);
                candidate_boards.push_back(&new_candidate_board);
            }
        }
    }

    print_board(board);
}

pub fn print_board(board: &Board) {
    let board_symbols: Vec<usize> = board
        .iter()
        .map(|maybe_num| maybe_num.unwrap_or(0))
        .collect();

    let board_lines = board_symbols.chunks(9);

    for line in board_lines {
        println!("{:?}", line);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parser::parse;

    #[test]
    fn empty_spaces_from_partial_board() {
        assert_eq!(
            vec![0, 2],
            get_empty_cells(&vec![None, Some(1), None, Some(2)])
        );
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
}
