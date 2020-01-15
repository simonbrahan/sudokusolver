use std::env;

mod input_parser;

fn main() {
    let input = env::args().nth(1).expect("Script accepts one argument");
    let game_board = input_parser::parse(&input).unwrap();

    let board_symbols: Vec<usize> = game_board
        .iter()
        .map(|maybe_num| maybe_num.unwrap_or(0))
        .collect();

    let board_lines = board_symbols.chunks(9);

    for line in board_lines {
        println!("{:?}", line);
    }
}
