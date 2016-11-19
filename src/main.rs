pub mod game_state;

fn main() {
    let mut board = game_state::GameState::new();
    board = board.place(game_state::Move::new(0, 0, game_state::Color::White));
    board = board.place(game_state::Move::new(1, 0, game_state::Color::White));
    board = board.place(game_state::Move::new(2, 0, game_state::Color::White));
    let end = board.win();
    println!("{}", board.print());
    println!("{:?}", end)
}
