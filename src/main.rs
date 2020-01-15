use std::env;

mod input_parser;

fn main() {
    let input = env::args().nth(1).expect("Script accepts one argument");
    let (game_board, max_symbol) = input_parser::parse(&input).unwrap();
    println!("{:?}", game_board);
    println!("{}", max_symbol);
}
