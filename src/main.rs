use std::env;

mod game;
mod input_parser;

fn main() {
    let input = env::args().nth(1).expect("Script accepts one argument");
    let game_board = input_parser::parse(&input).unwrap();

    game::print_board(&game_board);
}
