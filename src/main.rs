pub mod game_state;
pub mod monte_carlo;
extern crate rand;
extern crate petgraph;
use std::io;
use petgraph::Graph;

fn main() {
    let mut board = game_state::GameState::new();
    board = board.place(&game_state::Move::white_new(0, 0));
    board = board.place(&game_state::Move::black_new(2, 2));
    board = board.place(&game_state::Move::white_new(2, 0));
    board = board.place(&game_state::Move::black_new(1, 2));
    println!("{}", board.print());
    monte_carlo::tree_search(board);
    println!("Done");
}

fn play(){

}