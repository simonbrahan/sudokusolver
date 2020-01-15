pub fn parse(input: &str) -> Result<Vec<Option<usize>>, &str> {
    let game_board: Vec<Option<usize>> = input
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>()
        .split(',')
        .map(|num| num.parse().ok())
        .collect();

    if game_board.len() != 81 {
        return Err("Board must be 81 symbols in length");
    }

    if game_board
        .iter()
        .any(|maybe_num| maybe_num.unwrap_or(0) > 9)
    {
        return Err("Board may only contain symbols 1 to 9 or blank squares");
    }

    Ok(game_board)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
