#[derive(PartialEq)]
pub enum Side {
    None,
    Black,
    White,
}

#[derive(PartialEq)]
pub enum Type {
    Empty,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct Piece {
    piece_type: Type,
    side: Side,
}

impl Piece {
    pub fn to_string(&mut self) -> &str {
        match self.side {
            Side::White => match self.piece_type {
                Type::King => return "♚",
                Type::Queen => return "♛",
                Type::Rook => return "♜",
                Type::Bishop => return "♝",
                Type::Knight => return "♞",
                Type::Pawn => return "♟",
                Type::Empty => return "*",
            },
            Side::Black => match self.piece_type {
                Type::King => return "♔",
                Type::Queen => return "♕",
                Type::Rook => return "♖",
                Type::Bishop => return "♗",
                Type::Knight => return "♘",
                Type::Pawn => return "♙",
                Type::Empty => return "*",
            },
            Side::None => return "*",
        }
    }

    pub fn new(side: Side, piece_type: Type) -> Piece {
        Piece {
            piece_type: piece_type,
            side: side,
        }
    }
}
