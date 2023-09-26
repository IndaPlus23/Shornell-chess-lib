use std::fmt;

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


pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_colour: u8,
    board: Vec<Option<Piece>>,


}

pub fn new_piece(colour: u8, role: u8) -> Piece {
    Piece {colour: colour, role: role}
}



pub fn convert_to_int(_position: String) -> u8 {

    let mut position_vec: Vec<char> = Vec::new();
        position_vec = _position.split_whitespace().map(|x| x.parse::<char>().unwrap()).collect();

        let mut column: u8;
        let mut row: u8 = position_vec[1].to_digit(radix);

    if position_vec[0] == 'a' {
        column = 0;
    }
    if position_vec[0] == 'b' {
        column = 1;
    }
    if position_vec[0] == 'c' {
        column = 2;
    }
    if position_vec[0] == 'e' {
        column = 3;
    }
    if position_vec[0] == 'd' {
        column = 4;
    }
    if position_vec[0] == 'f' {
        column = 5;
    }
    if position_vec[0] == 'g' {
        column = 6;
    }
    if position_vec[0] == 'h' {
        column = 7;
    }


    if position_vec[1] == '8' {
        row = 0;
    }
    if position_vec[1] == '7' {
        row = 1;
    }
    if position_vec[1] == '6' {
        row = 2;
    }
    if position_vec[1] == '5' {
        row = 3;
    }
    if position_vec[1] == '4' {
        row = 4;
    }
    if position_vec[1] == '3' {
        row = 5;
    }
    if position_vec[1] == '2' {
        row = 6;
    }
    if position_vec[1] == '1' {
        row = 7;
    }


    return column + (row * 8);

}

pub fn convert_to_coordinates(_position: u8) -> String {

    let mut row: u8;
    let mut column: u8;
    
    if (_position >= 0 && _position <= 7 ) {
        row = 7;
    }
    if (_position >= 8 && _position <= 15 ) {
        row = 6;
    }
    if (_position >= 16 && _position <= 23 ) {
        row = 5;
    }
    if (_position >= 24 && _position <= 31 ) {
        row = 4;
    }
    if (_position >= 32 && _position <= 39 ) {
        row = 3;
    }
    if (_position >= 40 && _position <= 47 ) {
        row = 2;
    }
    if (_position >= 48 && _position <= 55 ) {
        row = 1;
    }
    if (_position >= 56 && _position <= 63 ) {
        row = 0;
    }

    let mut col = _position - (row * 8);

    let coordinates = String::new();
    
    if (column == 0) {
        coordinates = "a".to_string();
    }
    if (column == 1) {
        coordinates = "b".to_string();
    }
    if (column == 2) {
        coordinates = "c".to_string();
    }
    if (column == 3) {
        coordinates = "d".to_string();
    }
    if (column == 4) {
        coordinates = "e".to_string();
    }
    if (column == 5) {
        coordinates = "f".to_string();
    }
    if (column == 6) {
        coordinates = "g".to_string();
    }
    if (column == 7) {
        coordinates = "h".to_string();
    }

    coordinates = coordinates + row.to_string();

    return coordinates;

}


impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_colour: 0,
            board: vec![Some(Piece::new_piece(1,4)),  Some(new_piece(1,3)), Some(new_piece(1,2)), Some(new_piece(1,0)), Some(new_piece(1,1)), Some(new_piece(1,2)), Some(new_piece(1,3)), Some(new_piece(1,4)),
                    Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)), Some(new_piece(1,5)),
                    None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None,
                    Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)), Some(new_piece(0,5)),
                    Some(new_piece(0,4)), Some(new_piece(0,3)), Some(new_piece(0,2)), Some(new_piece(0,0)), Some(new_piece(0,1)), Some(new_piece(0,2)), Some(new_piece(0,3)), Some(new_piece(0,4)),],
    

        
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
     /*   let from = _from;
       let to = _to;
       
       get_possible_moves
       */
      None
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
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
       /* 
        let mut position_vec: Vec<u8> = Vec::new();
        position_vec = line2.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        
        let mut possible_moves_string: String = String::new();
        let mut possible_moves_vector: Vec<u8> = Vec::new();

        if board[position_vec[0]][position_vec[1]].role == 5 && board[position_vec[0]][position_vec[1]].colour == 0 {
            possible_moves_string[0] = ((position_vec[0] - 1) as String) + ((position_vec[1] as String));
            possible_moves_string[1] = ((position_vec[0] - 2) as String) + ((position_vec[1] as String));
        }
        if board[position_vec[0]][position_vec[1]].role == 5 && board[position_vec[0]][position_vec[1]].colour == 1 {
            possible_moves_string[0] = ((position_vec[0] + 1) as String) + ((position_vec[1] as String));
            possible_moves_string[1] = ((position_vec[0] + 2) as String) + ((position_vec[1] as String));
        }
        */

        let mut possible_moves_vec: Vec<String> = Vec::new();

        let mut position: u8 = convert_to_int(_position);


        if (self.board.get(position as usize).role == 5 && self.board[position as usize].colour == 0) {
            possible_moves_vec.push(convert_to_coordinates(position - 8));
            possible_moves_vec.push(convert_to_coordinates(position - 16));
        }
        if (self.board[position as usize].role == 5 && self.board[position as usize].colour == 1) {
            possible_moves_vec.push(convert_to_coordinates(position + 8));
            possible_moves_vec.push(convert_to_coordinates(position + 16));
        }

        if (self.board[position as usize].role == 4 && self.board[position as usize].colour == 0) {
            for i in 0..7 {
                possible_moves_vec.push(convert_to_coordinates(position - (8*i)));
                possible_moves_vec.push(convert_to_coordinates(position + (8*i)));
                possible_moves_vec.push(convert_to_coordinates(position - i));
                possible_moves_vec.push(convert_to_coordinates(position + i));
            }
            
        }
        if (self.board[position as usize].role == 4 && self.board[position as usize].colour == 1) {
            for i in 0..7 {
                possible_moves_vec.push(convert_to_coordinates(position - (8*i)));
                possible_moves_vec.push(convert_to_coordinates(position + (8*i)));
                possible_moves_vec.push(convert_to_coordinates(position - i));
                possible_moves_vec.push(convert_to_coordinates(position + i));
            }
        }

        if (self.board[position as usize].role == 3 && self.board[position as usize].colour == 0) {
         
                possible_moves_vec.push(convert_to_coordinates(position - (16+1)));
                possible_moves_vec.push(convert_to_coordinates(position - (16-1)));
                possible_moves_vec.push(convert_to_coordinates(position + (16+1)));
                possible_moves_vec.push(convert_to_coordinates(position + (16-1)));

                possible_moves_vec.push(convert_to_coordinates(position - (2+8)));
                possible_moves_vec.push(convert_to_coordinates(position - (2-8)));
                possible_moves_vec.push(convert_to_coordinates(position + (2+8)));
                possible_moves_vec.push(convert_to_coordinates(position + (2-8)));

            
            
        }
        if (self.board[position as usize].role == 3 && self.board[position as usize].colour == 1) {

                possible_moves_vec.push(convert_to_coordinates(position - (16+1)));
                possible_moves_vec.push(convert_to_coordinates(position - (16-1)));
                possible_moves_vec.push(convert_to_coordinates(position + (16+1)));
                possible_moves_vec.push(convert_to_coordinates(position + (16-1)));

                possible_moves_vec.push(convert_to_coordinates(position - (2+8)));
                possible_moves_vec.push(convert_to_coordinates(position - (2-8)));
                possible_moves_vec.push(convert_to_coordinates(position + (2+8)));
                possible_moves_vec.push(convert_to_coordinates(position + (2-8)));
        }

        if (self.board[position as usize].role == 2 && self.board[position as usize].colour == 0) {
         
            possible_moves_vec.push(convert_to_coordinates(position - (16+1)));
            possible_moves_vec.push(convert_to_coordinates(position - (16-1)));
            possible_moves_vec.push(convert_to_coordinates(position + (16+1)));
            possible_moves_vec.push(convert_to_coordinates(position + (16-1)));

            possible_moves_vec.push(convert_to_coordinates(position - (2+8)));
            possible_moves_vec.push(convert_to_coordinates(position - (2-8)));
            possible_moves_vec.push(convert_to_coordinates(position + (2+8)));
            possible_moves_vec.push(convert_to_coordinates(position + (2-8)));

        
        
        }
        if (self.board[position as usize].role == 2 && self.board[position as usize].colour == 1) {
            
            for i in 0..7 {
                possible_moves_vec.push(convert_to_coordinates(position - ((8+1))));

            } 



        }

        return Some(possible_moves_vec);
    }

    pub fn print_game(&self) {
      /* * for row in self.board.iter() {
            for col in row.iter() {
                if self.board[row][col].is_none() {
                    print!("* ");
                }
                print!("{}", col.role);
             
            }
        
        }
        */
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
        for square in self.board.iter() {
                match square {
                    None => print!("* "),
                    Some(piece) => print!("{:?} ", piece.role)   
                }
            
            print!("\n")
        }
        
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

        let game = Game::new();

        println!("{:?}", game);


        game.print_game();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    
}
