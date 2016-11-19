const BOARD_WIDTH : usize = 3;
const BOARD_HEIGHT : usize = 3;

#[derive(Debug, Copy, Clone)]
pub struct GameState{
    board : [[Color; BOARD_WIDTH]; BOARD_HEIGHT]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Empty,
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    color : Color,
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum End{
    Ongoing,
    Victory(Color),
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
                    [Color::Empty, Color::Empty, Color::Empty]]
        }
    }

    pub fn place(&self, game_move : Move) -> Self{
        let mut copy = self.clone();
        copy.board[game_move.x][game_move.y] = game_move.color;
        return copy;
    }

    fn legal(&self, game_move: Move) -> bool{
        if !game_move.in_bounds(){
            return false;
        }

        let tile = self.board[game_move.x][game_move.y];
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

    pub fn linear_match(&self, stepX : i32, stepY : i32, color : Color) -> bool{
        for i in 1..3{
            let x = (i * stepX) as usize;
            let y = (i * stepY) as usize;
            let value = self.board[x][y];
            println!("Im checking {} and {}", x, y);
            if value != color{
                return false
            }
        }
        return true;
    }

    pub fn win(&self) -> End{
        if self.linear_match(1, 0, Color::White) 
            || self.linear_match(0, 1, Color::White)
            || self.linear_match(1, 1, Color::White){
                return End::Victory(Color::White);
            }
        if self.linear_match(1, 0, Color::Black) 
            || self.linear_match(0, 1, Color::Black)
            || self.linear_match(1, 1, Color::Black){
                return End::Victory(Color::Black)
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
                        &Color::White => "O",
                        &Color::Black => "X"
                    };
                string.push_str(tile_str);
                string.push_str("|")
            }
            string.push_str("\n");
        }
        return string;
    }
}
