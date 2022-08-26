#[derive(PartialEq)]
pub enum Side {
    None,
    Black,
    White,
}

impl Side {
    pub fn to_string(&self) -> &str {
        match self {
            Side::Black => "Black",
            Side::White => "White",
            Side::None => "Empty",
        }
    }
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

impl Type {
    pub fn to_string(&self) -> &str {
        match self {
            Type::Empty => "Empty",
            Type::King => "King",
            Type::Queen => "Queen",
            Type::Rook => "Rook",
            Type::Bishop => "Bishop",
            Type::Knight => "Knight",
            Type::Pawn => "Pawn",
        }
    }
}

#[derive(PartialEq)]
pub struct Piece {
    pub piece_type: Type,
    pub side: Side,
}

impl Piece {
    pub fn to_string(&self) -> &str {
        match self.side {
            Side::White => match self.piece_type {
                Type::King => return "♚",
                Type::Queen => return "♛",
                Type::Rook => return "♜",
                Type::Bishop => return "♝",
                Type::Knight => return "♞",
                Type::Pawn => return "♟",
                Type::Empty => return "·",
            },
            Side::Black => match self.piece_type {
                Type::King => return "♔",
                Type::Queen => return "♕",
                Type::Rook => return "♖",
                Type::Bishop => return "♗",
                Type::Knight => return "♘",
                Type::Pawn => return "♙",
                Type::Empty => return "·",
            },
            Side::None => return "·",
        }
    }

    pub fn clone(piece: &Piece) -> Piece {
        Piece {
            piece_type: match piece.piece_type {
                Type::Empty => Type::King,
                Type::King => Type::King,
                Type::Queen => Type::Queen,
                Type::Rook => Type::Rook,
                Type::Bishop => Type::Bishop,
                Type::Knight => Type::Knight,
                Type::Pawn => Type::Pawn,
            },
            side: match piece.side {
                Side::None => Side::None,
                Side::Black => Side::Black,
                Side::White => Side::White,
            },
        }
    }

    pub fn empty() -> Piece {
        Piece {
            piece_type: Type::Empty,
            side: Side::None,
        }
    }

    pub fn new(side: Side, piece_type: Type) -> Piece {
        Piece {
            piece_type: piece_type,
            side: side,
        }
    }

    pub fn new_for_str(moniker: &str) -> Piece {
        match moniker {
            "wr" => Piece::new(Side::White, Type::Rook),
            "wkn" => Piece::new(Side::White, Type::Knight),
            "wb" => Piece::new(Side::White, Type::Bishop),
            "wq" => Piece::new(Side::White, Type::Queen),
            "wk" => Piece::new(Side::White, Type::King),
            "wp" => Piece::new(Side::White, Type::Pawn),
            "br" => Piece::new(Side::Black, Type::Rook),
            "bkn" => Piece::new(Side::Black, Type::Knight),
            "bb" => Piece::new(Side::Black, Type::Bishop),
            "bq" => Piece::new(Side::Black, Type::Queen),
            "bk" => Piece::new(Side::Black, Type::King),
            "bp" => Piece::new(Side::Black, Type::Pawn),
            "e" => Piece::new(Side::None, Type::Empty),
            &_ => panic!("no moniker with {moniker}"),
        }
    }
}
