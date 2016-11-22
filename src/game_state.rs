const BOARD_WIDTH : usize = 3;
const BOARD_HEIGHT : usize = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GameState{
    board : [[Color; BOARD_WIDTH]; BOARD_HEIGHT],
    pub player : Color
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Empty,
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub color : Color,
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum End{
    Ongoing,
    Victory(Color),
    Tie
}

impl Move{
    pub fn new(nx : usize, ny : usize, ncolor : Color) -> Self{
        Move{
            color : ncolor,
            x: nx,
            y: ny
        }
    }

    pub fn white_new(nx : usize, ny : usize) -> Self{
        Move::new(nx, ny, Color::White) 
    }

    pub fn black_new(nx : usize, ny : usize) -> Self{
        Move::new(nx, ny, Color::Black)
    }

    pub fn from_int(nx : i32, ny : i32, ncolor : Color) -> Self{
        Move{
            color : Color::White,
            x : nx as usize,
            y : ny as usize
        }
    }

    fn in_bounds(&self) -> bool{
        (self.x <= BOARD_WIDTH && self.x  >= 0 && self.y <= BOARD_HEIGHT && self.y >= 0)
    }
}

impl GameState{
    pub fn new() -> Self{
        GameState{ 
            board: [[Color::Empty, Color::Empty, Color::Empty],
                    [Color::Empty, Color::Empty, Color::Empty],
                    [Color::Empty, Color::Empty, Color::Empty]],
            player : Color::White
        }
    }

    pub fn place(&self, game_move : &Move) -> Self{
        let mut copy = self.clone();
        copy.board[game_move.y][game_move.x] = game_move.color;
        let next_player = 
        match game_move.color{
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::White 
        };

        copy.player = next_player;
        return copy;
    }

    pub fn legal(&self, game_move: Move) -> bool{
        if !game_move.in_bounds(){
            return false;
        }

        let tile = self.board[game_move.y][game_move.x];
        if tile != Color::Empty{
            return false;
        }
        else{
            return true;
        }
    }

    pub fn legal_moves(&self, color : Color) -> Vec<Move>{
        let mut moves = Vec::new();
        for y in 0..BOARD_HEIGHT{
            for x in 0..BOARD_WIDTH{
                let potential_move = Move::new(x, y, color);
                if self.legal(potential_move){
                    moves.push(potential_move);
                }
            }
        }
        return moves;
    }

    pub fn linear_match(&self, start_x : i32, start_y : i32, stepX : i32, stepY : i32, color : Color) -> bool{
        for i in 0..3{
            let x = (start_x + i * stepX) as usize;
            let y = (start_y + i * stepY) as usize;
            let value = self.board[y][x];
            if value != color{
                return false;
            }
        }
        return true;
    }


    fn color_win(&self, color : Color) -> bool{

        if self.linear_match(0, 0, 1, 1, color) || self.linear_match(2, 0, -1, 1, color){
            return true;
        }

        for i in 0..3{
            //rows
            if self.linear_match(0, i, 1, 0, color){
                return true;
            }

            //column
            if self.linear_match(i, 0, 0, 1, color){
                return true;
            }
        }

        return false;

    }

    pub fn win(&self) -> End{
        if self.color_win(Color::White) {
            return End::Victory(Color::White);
        }
        if self.color_win(Color::Black) {
            return End::Victory(Color::Black);
        }
        let mut no_empty = true;
        for column in self.board.iter(){
            for &tile in column{
                if tile == Color::Empty{
                    no_empty = false;
                }
            }
        }
        if no_empty {
            return End::Tie;
        }
        return End::Ongoing;
    }

    pub fn print(&self) -> String{
        let mut string = String::from("\n");
        for column in self.board.iter(){
            for tile in column{
                string.push_str("|");
                let tile_str =
                    match tile {
                        &Color::Empty => " ",
                        &Color::White => "X",
                        &Color::Black => "O"
                    };
                string.push_str(tile_str);
                string.push_str("|")
            }
            string.push_str("\n");
        }
        return string;
    }
}

#[cfg(test)]
mod test{
    use super::*; 

    #[test]
    fn legality(){
        let board = GameState::new();
        assert_eq!(board.legal(Move::black_new(4, 4)), false);
        assert_eq!(board.legal(Move::white_new(0, 0)), true);

        let new_board = board.place(Move::white_new(0, 0));
        assert_eq!(new_board.legal(Move::black_new(0, 0)), false);
    }

    fn white_victory(end : End) -> bool{
        match end{
            End::Victory(Color::White) => true,
            _ => false
        }
    }

    #[test]
    fn white_win(){
        let mut board = GameState::new();

        board = board.place(Move::white_new(0, 0));
        board = board.place(Move::white_new(1, 0));
        board = board.place(Move::white_new(2, 0));

        assert!(white_victory(board.win()));

        board = GameState::new();

        board = board.place(Move::white_new(0, 0));
        board = board.place(Move::white_new(0, 1));
        board = board.place(Move::white_new(0, 2));

        assert!(white_victory(board.win()));

        board = GameState::new();

        board = board.place(Move::white_new(0, 2));
        board = board.place(Move::white_new(1, 2));
        board = board.place(Move::white_new(2, 2));

        assert!(white_victory(board.win()));

        board = GameState::new();

        board = board.place(Move::white_new(2,0));
        board = board.place(Move::white_new(1,1));
        board = board.place(Move::white_new(0,2));

        assert!(white_victory(board.win()));
    }

    #[test]
    #[should_panic]
    fn white_not_win(){

        let mut board = GameState::new();
        board = board.place(Move::white_new(0, 0));
        board = board.place(Move::white_new(1, 0));
        println!("{}", board.print());
        assert!(white_victory(board.win()));
    }

    #[test]
    fn tie(){
        let mut board = GameState::new();
        board = board.place(Move::black_new(0, 0));
        board = board.place(Move::white_new(1, 0));
        board = board.place(Move::black_new(2, 0));

        board = board.place(Move::white_new(0, 1));
        board = board.place(Move::white_new(1, 1));
        board = board.place(Move::black_new(2, 1));

        board = board.place(Move::white_new(0, 2));
        board = board.place(Move::black_new(1, 2));
        board = board.place(Move::white_new(2, 2));

        let bwin = board.win();
        let win =
        match bwin{
            End::Tie => true,
            _ => false
        };

        assert!(win);
    }
}
