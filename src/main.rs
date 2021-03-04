use std::env;

mod game;

fn main() {
    let input = env::args().nth(1).expect("Script accepts one argument");
    let game_board = game::parse(&input).unwrap();

    if let Some(solved) = game::solve(&game_board) {
        game::print_board(&solved);
    } else {
        println!("Unsolveable");
    }
}
