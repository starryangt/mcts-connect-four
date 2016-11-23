const BOARD_WIDTH : usize = 7;
const BOARD_HEIGHT : usize = 6;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GameState{
    board : [[Color; BOARD_HEIGHT]; BOARD_WIDTH],
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
    pub x: usize
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum End{
    Ongoing,
    Victory(Color),
    Tie
}

impl Move{
    pub fn new(nx : usize, ncolor : Color) -> Self{
        Move{
            color : ncolor,
            x: nx
        }
    }

    pub fn white_new(nx : usize) -> Self{
        Move::new(nx, Color::White) 
    }

    pub fn black_new(nx : usize) -> Self{
        Move::new(nx, Color::Black)
    }

    fn in_bounds(&self) -> bool{
        (self.x <= BOARD_WIDTH && self.x  >= 0)
    }
}

fn push_back(item : Color, column : &mut [Color; 6]) -> usize{
    for i in (0..column.len()).rev(){
        if (column[i] == Color::Empty){
            return i;
        }
    }
    return 0;
}

impl GameState{
    pub fn new() -> Self{
        GameState{ 
            board: [[Color::Empty; BOARD_HEIGHT]; BOARD_WIDTH],
            player : Color::White
        }
    }

    pub fn place(&self, game_move : &Move) -> Self{
        let mut copy = self.clone();
        if !self.legal(&game_move){
            return copy;
        }

        let mut column = copy.board[game_move.x];
        let y = push_back(game_move.color, &mut column);
        copy.board[game_move.x][y] = game_move.color;
        //copy.board[game_move.y][game_move.x] = game_move.color;
        let next_player = 
        match game_move.color{
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::White 
        };

        copy.player = next_player;
        return copy;
    }

    pub fn legal(&self, game_move: &Move) -> bool{
        if !game_move.in_bounds(){
            return false;
        }

        let column = self.board[game_move.x];
        column.iter().fold(0, |sum, next| if next == &Color::Empty { sum }  else { sum + 1 } ) < BOARD_HEIGHT
    }

    pub fn legal_moves(&self, color : Color) -> Vec<Move>{
        let mut moves = Vec::new();
        for x in 0..BOARD_WIDTH{
            let potential_move = Move::new(x, color);
            if self.legal(&potential_move){
                moves.push(potential_move);
            }
        }
        return moves;
    }

    pub fn linear_match(&self, start_x : i32, start_y : i32, stepX : i32, stepY : i32, color : Color) -> bool{
        for i in 0..4{
            let x = (start_x + i * stepX) as usize;
            let y = (start_y + i * stepY) as usize;
            //bound checking
            if x < 0 || x >= BOARD_HEIGHT || y < 0 || y >= BOARD_WIDTH{
                return false;
            }
            let value = self.board[y][x];
            if value != color{
                return false;
            }
        }
        return true;
    }

    pub fn is_line_start(&self, x : i32, y : i32, color : Color) -> bool{
        self.linear_match(x, y, 1, 0, color) 
        || self.linear_match(x, y, 0, 1, color) 
        || self.linear_match(x, y, 1, 1, color)
        || self.linear_match(x, y, 1, -1, color)
    }

    fn color_win(&self, color : Color) -> bool{
        for x in 0..BOARD_HEIGHT + 1{
            for y in 0..BOARD_WIDTH + 1{
                if self.is_line_start(y as i32, x as i32, color){
                    return true;
                }
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
        for x in 0..BOARD_HEIGHT{
            for y in 0..BOARD_WIDTH{
                string.push_str("|");
                let tile = self.board[y][x];
                let tile_str =
                    match tile {
                        Color::Empty => " ",
                        Color::White => "X",
                        Color::Black => "O"
                    };
                string.push_str(tile_str);
                string.push_str("|")
            }
            string.push_str("\n");
        }
        string.push_str("|1||2||3||4||5||6||7|");
        
        return string;
    }
}

#[cfg(test)]
mod test{
    use super::*; 

}
