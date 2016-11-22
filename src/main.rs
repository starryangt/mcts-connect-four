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
        let v: Vec<usize> = input.split(',').map(|x| x.trim().parse::<usize>().unwrap()).collect();
        let x = v[0];
        let y = v[1];
        let mv = game_state::Move::white_new(x, y);
        board = board.place(&mv);
        print_board(&board);
        let best_move = monte_carlo::tree_search(board);
    }
    print_board(&board);
    println!("Result: {:?}", board.win());
}

fn print_board(board : &game_state::GameState){
    println!("{}", board.print());
}