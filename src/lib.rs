use std::{fmt, f32::consts::E};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
    CheckMate
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */
/* 
enum Piece_role {
    King = 0, Queen = 1, Bishop = 2, Knight = 3, Rook = 4, Pawn = 5
}

enum Colour {
    White = 0,
    Black = 1
}
*/
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    colour: u8,
    role: u8
} 

impl Piece {
    fn description (&self) {
        print!("Colour: {}, ", self.colour);
        println!("Role: {}", self.role);
    }

    fn new_piece(colour: u8, role: u8) -> Piece {
        Piece {colour: colour, role: role}
    }

}

#[derive(Clone)]
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_colour: u8,
    board: Vec<Option<Piece>>,


}

pub fn new_piece(colour: u8, role: u8) -> Piece {
    Piece {colour: colour, role: role}
}

pub fn check_edge_overflow (_position: i16) -> bool {
   if _position >= 0 && _position <= 25 &&
    _position >= 34 && _position <= 37 &&
    _position >= 46 && _position <= 49 &&
    _position >= 58 && _position <= 61 &&
    _position >= 70 && _position <= 73 &&
    _position >= 82 && _position <= 85 &&
    _position >= 94 && _position <= 97 &&
    _position >= 106 && _position <= 109 &&
    _position >= 118 && _position <= 143 {
        return false;
    }
    else {
        return true;
    }
}



pub fn convert_to_int(_position: String) -> i16 {

    let mut position_vec: Vec<char> = Vec::new();
        position_vec = _position.split_whitespace().map(|x| x.parse::<char>().unwrap()).collect();

        let mut column: i16;
        let mut row: i16 = position_vec[1] as i16;

    if position_vec[0] == 'a' {
        column = 2;
    }
    if position_vec[0] == 'b' {
        column = 3;
    }
    if position_vec[0] == 'c' {
        column = 4;
    }
    if position_vec[0] == 'e' {
        column = 5;
    }
    if position_vec[0] == 'd' {
        column = 6;
    }
    if position_vec[0] == 'f' {
        column = 7;
    }
    if position_vec[0] == 'g' {
        column = 8;
    }
    if position_vec[0] == 'h' {
        column = 9;
    }
    else {
        column = 0;
    }


    if position_vec[1] == '8' {
        row = 2;
    }
    if position_vec[1] == '7' {
        row = 3;
    }
    if position_vec[1] == '6' {
        row = 4;
    }
    if position_vec[1] == '5' {
        row = 5;
    }
    if position_vec[1] == '4' {
        row = 6;
    }
    if position_vec[1] == '3' {
        row = 7;
    }
    if position_vec[1] == '2' {
        row = 8;
    }
    if position_vec[1] == '1' {
        row = 9;
    }
    else {
        row = 0;
    }


    return column + (row * 8);

}

pub fn convert_to_coordinates(_position: i16) -> String {

    let mut row: i16;
    let column: i16;
    
    if _position >= 26 && _position <= 33  {
        row = 8;
    }
    if _position >= 38 && _position <= 45  {
        row = 7;
    }
    if _position >= 50 && _position <= 57  {
        row = 6;
    }
    if _position >= 62 && _position <= 69  {
        row = 5;
    }
    if _position >= 74 && _position <= 81  {
        row = 4;
    }
    if _position >= 86 && _position <= 93  {
        row = 3;
    }
    if _position >= 98 && _position <= 105  {
        row = 2;
    }
    if _position >= 110 && _position <= 112  {
        row = 1;
    }
    else {
        row = 0;
    }

    column = _position - (row * 8);

    let mut coordinates = String::new();
    
    if column == 0 {
        coordinates = "a".to_string();
    }
    if column == 1 {
        coordinates = "b".to_string();
    }
    if column == 2 {
        coordinates = "c".to_string();
    }
    if column == 3 {
        coordinates = "d".to_string();
    }
    if column == 4 {
        coordinates = "e".to_string();
    }
    if column == 5 {
        coordinates = "f".to_string();
    }
    if column == 6 {
        coordinates = "g".to_string();
    }
    if column == 7 {
        coordinates = "h".to_string();
    }
    else {
        coordinates = "a".to_string();
    }

    coordinates = coordinates + &row.to_string();

    return coordinates;

}


impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_colour: 0,
            board: vec![None, None, None, None, None, None, None, None, None, None,None, None,
                        None, None, None, None, None, None, None, None, None, None,None, None,
                        None, None, Some(Piece::new_piece(1,4)),  Some(new_piece(1,3)), Some(new_piece(1,2)), Some(new_piece(1,0)), Some(new_piece(1,1)), Some(new_piece(1,2)), Some(new_piece(1,3)), Some(new_piece(1,4)), None, None,
                        None, None, Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), None, None,
                        None, None, Some(new_piece(0,4)), Some(new_piece(0,3)), Some(new_piece(0,2)), Some(new_piece(0,0)), Some(new_piece(0,1)), Some(new_piece(0,2)), Some(new_piece(0,3)), Some(new_piece(0,4)), None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,],
    

        
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
    
        let from = _from;
        let to = _to;

        self.board[convert_to_int(to.to_string()) as usize] =  self.board[convert_to_int(from.to_string()) as usize].clone();
        
        self.board[convert_to_int(from.to_string()) as usize] = None;
       
       return Some(self.state);
       
      
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {

        let mut possible_moves_vec: Vec<String> = Vec::new();

        let position: i16 = convert_to_int(_position.to_string());

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 5 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 0 {
           if check_edge_overflow(position - 12) {
            possible_moves_vec.push(convert_to_coordinates(position - 12));
           }
           if check_edge_overflow(position - 24) {
            possible_moves_vec.push(convert_to_coordinates(position - 24));
           }
        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 5 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 1 {
            if check_edge_overflow(position + 12) {
            possible_moves_vec.push(convert_to_coordinates(position + 12));
            }
            if check_edge_overflow(position + 24) {
            possible_moves_vec.push(convert_to_coordinates(position + 24));
            }
        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 4 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 0 {
            for i in 0..7 {
                if check_edge_overflow(position - (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position + (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position - i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - i));
            }
            for i in 0..7 {
                if check_edge_overflow(position + i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + i));
            }
            
        }
        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 4 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 1 {
            for i in 0..7 {
                if check_edge_overflow(position - (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position + (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position - i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - i));
            }
            for i in 0..7 {
                if check_edge_overflow(position + i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + i));
            }
        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 3 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 0 {

            if check_edge_overflow(position - (24 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position - (24+1)));
            }
            if check_edge_overflow(position - (24 - 1)) {
                possible_moves_vec.push(convert_to_coordinates(position - (24-1)));
            }
            if check_edge_overflow(position + (24 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (24+1)));
            }
            if check_edge_overflow(position + (24 - 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (24-1)));
            }
            if check_edge_overflow(position - (2 + 12)) {
                possible_moves_vec.push(convert_to_coordinates(position - (2+12)));
            }
            if check_edge_overflow(position - (2 - 12)) {
                possible_moves_vec.push(convert_to_coordinates(position - (2 - 12)));
            }
            if check_edge_overflow(position + (2 + 12)) {
                possible_moves_vec.push(convert_to_coordinates(position + (2+12)));
            }
            if check_edge_overflow(position + (2 - 12)) {
                possible_moves_vec.push(convert_to_coordinates(position + (2-12)));
            }

            
            
        }
        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 3 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 1 {
            if check_edge_overflow(position - (24 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position - (24+1)));
            }
            if check_edge_overflow(position - (24 - 1)) {
                possible_moves_vec.push(convert_to_coordinates(position - (24-1)));
            }
            if check_edge_overflow(position + (24 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (24+1)));
            }
            if check_edge_overflow(position + (24 - 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (24-1)));
            }
            if check_edge_overflow(position - (2 + 12)) {
                possible_moves_vec.push(convert_to_coordinates(position - (2+12)));
            }
            if check_edge_overflow(position - (2 - 12)) {
                possible_moves_vec.push(convert_to_coordinates(position - (2 - 12)));
            }
            if check_edge_overflow(position + (2 + 12)) {
                possible_moves_vec.push(convert_to_coordinates(position + (2+12)));
            }
            if check_edge_overflow(position + (2 - 12)) {
                possible_moves_vec.push(convert_to_coordinates(position + (2-12)));
            }
        }


        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 2 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 1 {
            let mut i: i16 = 0;
            while i < 7 {
                if check_edge_overflow(position - (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12+1))));
                
                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position - (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12-1))));
                  
                i += 1;

            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12+1))));
                    
                i += 1;

            
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12-1))));

                i += 1;

            } 

        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 2 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 0 {
            let mut i: i16 = 0;
            while i < 7 {
                if check_edge_overflow(position - (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12+1))));
                
                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position - (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12-1))));   

                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12+1))));
                    
                i += 1;            
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12-1))));

                i += 1;
            } 


        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 1 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 0 {
            let mut i: i16 = 0;
            while i < 7 {
                if check_edge_overflow(position - (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12+1))));
                
                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position - (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12-1))));   
                    
                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12+1))));
                    
                i += 1;            
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12-1))));

                i += 1;
            } 

            for i in 0..7 {
                if check_edge_overflow(position - (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position + (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position - i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - i));
            }
            for i in 0..7 {
                if check_edge_overflow(position + i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + i));
            }

        }
        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 1 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 1 {
            let mut i: i16 = 0;
            while i < 7 {
                if check_edge_overflow(position - (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12+1))));
                
                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position - (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (i*(12-1))));   
                    
                i += 1;
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12+1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12+1))));
                    
                i += 1;            
            }

            i = 0;

            while i < 7 {
                if check_edge_overflow(position + (i*(12-1))) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (i*(12-1))));

                i += 1;
            } 


            for i in 0..7 {
                if check_edge_overflow(position - (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position + (12*i)) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + (8*i)));
            }
            for i in 0..7 {
                if check_edge_overflow(position - i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position - i));
            }
            for i in 0..7 {
                if check_edge_overflow(position + i) == false {
                    break;
                }
                possible_moves_vec.push(convert_to_coordinates(position + i));
            }

        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 0 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 1 {
            if check_edge_overflow(position - 1) {
                possible_moves_vec.push(convert_to_coordinates(position - 1));
            }
            if check_edge_overflow(position - (12 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position - (8+1)));
            }
            if check_edge_overflow(position - 12 ) {
                possible_moves_vec.push(convert_to_coordinates(position - (12)));
            }
            if check_edge_overflow(position - (12 - 1) ) {
            possible_moves_vec.push(convert_to_coordinates(position - (12-1)));
            }
            if check_edge_overflow(position + 1) {
                possible_moves_vec.push(convert_to_coordinates(position + 1));
            }
            if check_edge_overflow(position + (12 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (12 + 1)));
            }
            if check_edge_overflow(position + (12)) {
                possible_moves_vec.push(convert_to_coordinates(position + (12)));
            }
            if check_edge_overflow(position + (12 - 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (12-1)));
            }
        }

        if (*self.board.get(position as usize).unwrap()).as_ref().unwrap().role == 0 && (*self.board.get(position as usize).unwrap()).as_ref().unwrap().colour == 0 {
            if check_edge_overflow(position - 1) {
                possible_moves_vec.push(convert_to_coordinates(position - 1));
            }
            if check_edge_overflow(position - (12 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position - (12+1)));
            }
            if check_edge_overflow(position - 12 ) {
                possible_moves_vec.push(convert_to_coordinates(position - (12)));
            }
            if check_edge_overflow(position - (12 - 1) ) {
                possible_moves_vec.push(convert_to_coordinates(position - (12-1)));
            }
            if check_edge_overflow(position + 1) {
                possible_moves_vec.push(convert_to_coordinates(position + 1));
            }
            if check_edge_overflow(position + (12 + 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (12 + 1)));
            }
            if check_edge_overflow(position + (12)) {
                possible_moves_vec.push(convert_to_coordinates(position + (12)));
            }
            if check_edge_overflow(position + (12 - 1)) {
                possible_moves_vec.push(convert_to_coordinates(position + (12-1)));
            }
        }

        return Some(possible_moves_vec);
    }

    pub fn print_game(&self) {
        let mut flag = 0;
        for square in self.board.iter() {
            match square {
                None => print!("* "),
                Some(piece) => print!("{:?} ", piece.role)   
            }
            flag += 1;
            if flag == 12 {
            print!("\n");
            flag =0;
            }
    }
    
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
/// 



impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
  
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let mut game = Game::new();

        println!("{:?}", game);


        game.print_game();

        assert_eq!(game.get_game_state(), GameState::InProgress);

        
        game.make_move("a2", "a4");

        game.print_game();
        
    }

    
}
