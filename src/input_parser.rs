use std::collections::HashMap;

pub fn parse(input: &str) -> Result<(Vec<Option<usize>>, usize), &str> {
    let valid_board_lengths: HashMap<usize, usize> = [
        (1, 1),
        (4, 2),
        (9, 3),
        (16, 4),
        (25, 5),
        (36, 6),
        (49, 7),
        (64, 8),
        (81, 9),
    ]
    .iter()
    .cloned()
    .collect();

    let game_board: Vec<Option<usize>> = input.split(',').map(|num| num.parse().ok()).collect();

    let maybe_max_symbol = valid_board_lengths.get(&game_board.len());

    if maybe_max_symbol.is_none() {
        return Err("Game board must be a square of max size 9");
    }

    let max_symbol = *maybe_max_symbol.unwrap();

    if game_board
        .iter()
        .any(|maybe_num| maybe_num.unwrap_or(0) > max_symbol)
    {
        return Err("Board contains a symbol that is too large");
    }

    Ok((game_board, max_symbol))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_single_square() {
        assert_eq!(Ok((vec![None], 1)), parse(" "));
    }

    #[test]
    fn parse_single_square() {
        assert_eq!(Ok((vec![Some(1)], 1)), parse("1"));
    }

    #[test]
    fn parse_empty_three_square() {
        assert_eq!(
            Ok((
                vec![None, None, None, None, None, None, None, None, None],
                3
            )),
            parse(",,,,,,,,")
        );
    }

    #[test]
    fn parse_three_square() {
        assert_eq!(
            Ok((
                vec![
                    Some(1),
                    None,
                    Some(2),
                    None,
                    None,
                    None,
                    Some(3),
                    None,
                    None
                ],
                3
            )),
            parse("1,,2,,,,3,,")
        );
    }

    #[test]
    fn parse_board_wrong_size() {
        assert_eq!(
            Err("Game board must be a square of max size 9"),
            parse(",,,,")
        );
    }

    #[test]
    fn parse_board_too_large() {
        assert_eq!(
            Err("Game board must be a square of max size 9"),
            // Board is square, but is 100 blocks - too large
            parse(",,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,")
        );
    }

    #[test]
    fn parse_board_symbol_too_large() {
        assert_eq!(
            Err("Board contains a symbol that is too large"),
            parse("1,2,3,4,,,,,")
        );
    }
}
