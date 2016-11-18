pub mod game_state;

fn main() {
    let mut board = game_state::GameState::new();
    board = board.place(game_state::Move::new(0, 0));
    println!("Hello, world!");
}
