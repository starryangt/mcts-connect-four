const BOARD_WIDTH : usize = 3;
const BOARD_HEIGHT : usize = 3;

#[derive(Debug, Copy, Clone)]
pub struct GameState{
    board : [[i32; BOARD_WIDTH]; BOARD_HEIGHT]
}

#[derive(Debug, Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    color : Color,
    x: usize,
    y: usize
}

impl Move{
    pub fn new(nx : usize, ny : usize) -> Self{
        Move{
            color : Color::White,
            x: nx,
            y: ny
        }
    }

    pub fn from_int(nx : i32, ny : i32) -> Self{
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
            board: [[0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0]]
        }
    }

    pub fn place(&self, game_move : Move) -> Self{
        let mut copy = self.clone();
        copy.board[game_move.x][game_move.y] = 1;
        return copy;
    }

    fn legal(&self, game_move: Move) -> bool{
        if !game_move.in_bounds(){
            return false;
        }

        let tile = self.board[game_move.x][game_move.y];
        if tile > 0{
            return false;
        }
        else{
            return true;
        }
    }

    pub fn legal_moves(&self) -> Vec<Move>{
        let mut moves = Vec::new();
        for y in 0..BOARD_HEIGHT{
            for x in 0..BOARD_WIDTH{
                let potential_move = Move::new(x, y);
                if self.legal(potential_move){
                    moves.push(potential_move);
                }
            }
        }
        return moves;
    }

    fn linear_match(x : i32, y : i32, stepX : i32, stepY : i32){

    }

    pub fn win(&self) -> bool{
        unimplemented!()
    }
}
