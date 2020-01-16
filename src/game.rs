pub type Board = [Option<usize>];

use std::collections::HashSet;

fn get_empty_spaces(board: &Board) -> Vec<usize> {
    let mut out = Vec::<usize>::new();

    for (idx, space) in board.iter().enumerate() {
        if space.is_none() {
            out.push(idx);
        }
    }

    out
}

fn get_space_options(board: &Board, space_idx: usize) -> Vec<usize> {
    let mut taken = HashSet::new();

    // Get symbols taken by column
    let column = space_idx % 9;
    for cell in board.iter().skip(column).step_by(9) {
        if cell.is_some() {
            taken.insert(cell.unwrap());
        }
    }

    // Get symbols taken by row
    let row_start = space_idx - column;
    let after_row_end = row_start + 9;
    for cell in board.iter().take(after_row_end).skip(row_start) {
        if cell.is_some() {
            taken.insert(cell.unwrap());
        }
    }

    let available = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut out = Vec::new();
    for candidate in &available {
        if !taken.contains(&candidate) {
            out.push(*candidate);
        }
    }

    out
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parser::parse;

    #[test]
    fn empty_spaces_from_partial_board() {
        assert_eq!(
            vec![0, 2],
            get_empty_spaces(&vec![None, Some(1), None, Some(2)])
        );
    }

    #[test]
    fn space_options_from_full_board() {
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

        assert_eq!(Vec::<usize>::new(), get_space_options(&full_board, 0))
    }

    #[test]
    fn space_options_from_partial_column() {
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

        assert_eq!(vec![9], get_space_options(&board_with_partial_column, 25))
    }

    #[test]
    fn space_options_from_partial_row() {
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

        assert_eq!(vec![7], get_space_options(&board_with_partial_row, 55))
    }
}
