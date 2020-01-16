pub type Board = [Option<usize>];

fn get_empty_spaces(board: &Board) -> Vec<usize> {
    let mut out = Vec::<usize>::new();

    for (idx, space) in board.iter().enumerate() {
        if space.is_none() {
            out.push(idx);
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

    #[test]
    fn empty_spaces_from_partial_board() {
        assert_eq!(
            vec![0, 2],
            get_empty_spaces(&vec![None, Some(1), None, Some(2)])
        );
    }
}
