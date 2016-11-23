pub mod game_state;
pub mod monte_carlo;
extern crate rand;
extern crate time;
use std::io;

fn main() {
    play();
}

fn play(){
    let mut board = game_state::GameState::new();
    while !monte_carlo::victory(board.win()){
        print_board(&board);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("rip");
        let x = input.trim().parse::<usize>().unwrap();
        let mv = game_state::Move::white_new(x - 1);
        board = board.place(&mv);
        print_board(&board);
        let best_move = monte_carlo::tree_search(board);
        println!("{:?}", best_move);
        board = board.place(&best_move);
    }
    print_board(&board);
    println!("Result: {:?}", board.win());
    io::stdin().read_line(&mut String::new()).expect("idk");
}

fn print_board(board : &game_state::GameState){
    println!("{}", board.print());
}