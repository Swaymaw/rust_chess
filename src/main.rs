// bitboard is more useful when generating moves and stuff
type PiecePosition = u64;

fn bit_to_position(bit: PiecePosition) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    }
    else {
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
 
fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;
    // 2, 1
    return format!("{}{}", COL_MAP[column], row);
}

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58, 
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8, 
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33
];

fn bit_scan(bit: u64) -> usize {
    let remainder = (bit % 67) as usize;
    return MOD67TABLE[remainder];

}

// Color
// PieceType
// Piece Struct 


// Position/Square to piece

#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    White,
    Black
}

#[derive(Debug, PartialEq)]
enum PieceType {
    Pawn,
    Rook, 
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, PartialEq)]
struct Piece {
    position: PiecePosition,
    color: Color,
    piece_type: PieceType
}

// Square is either empty or occupied
#[derive(Debug)]
enum Square {
    Empty,
    Occupied(usize),
}

// Game type to own the data
struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>,
}

impl Game {

    fn push_piece_and_sqaure(&mut self, position: usize, color: Color,
                             piece_type: PieceType, index: &mut usize) {
        self.pieces.push(Piece {position: (1 as u64) << position, 
                                color: color, piece_type: piece_type});

        self.squares.push(Square::Occupied(*index));
        *index += 1;
    }
    fn push_empty_square(&mut self) {
        self.squares.push(Square::Empty);
    }

    fn initialize() -> Game {
        let mut game = Game {pieces: vec![], squares: vec![]};
        let  mut piece_index = 0;
        let color = Color::White;
        game.push_piece_and_sqaure(0, color, PieceType::Rook, &mut piece_index);
        game.push_piece_and_sqaure(1, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_sqaure(2, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_sqaure(3, color, PieceType::Queen, &mut piece_index);
        game.push_piece_and_sqaure(4, color, PieceType::King, &mut piece_index);
        game.push_piece_and_sqaure(5, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_sqaure(6, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_sqaure(7, color, PieceType::Rook, &mut piece_index);

        for i in 8..16 {
            game.push_piece_and_sqaure(i, color, PieceType::Pawn, &mut piece_index);
        }
        for i in 16..48 {
            game.push_empty_square();
        }
        let color = Color::Black;
        for i in 48..56 {
            game.push_piece_and_sqaure(i, color, PieceType::Pawn, &mut piece_index);
        }
        let offset = 56;
        game.push_piece_and_sqaure(0 + offset, color, PieceType::Rook, &mut piece_index);
        game.push_piece_and_sqaure(1 + offset, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_sqaure(2 + offset, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_sqaure(3 + offset, color, PieceType::Queen, &mut piece_index);
        game.push_piece_and_sqaure(4 + offset, color, PieceType::King, &mut piece_index);
        game.push_piece_and_sqaure(5 + offset, color, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_sqaure(6 + offset, color, PieceType::Knight, &mut piece_index);
        game.push_piece_and_sqaure(7 + offset, color, PieceType::Rook, &mut piece_index);

        game 
    }
    fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(&index_to_position(i)),
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            }

            if (i+1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear()
            }

        }
        board.insert_str(0, &temp);
        board
    }
}

impl Piece {
    fn to_string(&self) -> String {
        let mut result = match self.piece_type {
            PieceType::Pawn => "p ", 
            PieceType::Rook => "r ", 
            PieceType::Knight => "n ", 
            PieceType::Bishop => "b ", 
            PieceType::Queen => "q ", 
            PieceType::King => "k ", 
             
        }.to_string();
        if self.color == Color::White {
            result.make_ascii_uppercase();
        }
        result
    }
}

fn main() {
    let game = Game::initialize();
    println!("{}", game.to_string());
}