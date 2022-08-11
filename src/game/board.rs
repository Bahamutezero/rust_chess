use super::piece::Piece;
use super::piece::Side;
use super::piece::Type;

fn b_b() -> Piece {
    Piece::new(Side::Black, Type::Bishop)
}

fn b_p() -> Piece {
    Piece::new(Side::Black, Type::Pawn)
}

fn b_k() -> Piece {
    Piece::new(Side::Black, Type::King)
}

fn b_q() -> Piece {
    Piece::new(Side::Black, Type::Queen)
}

fn b_r() -> Piece {
    Piece::new(Side::Black, Type::Rook)
}

fn b_kn() -> Piece {
    Piece::new(Side::Black, Type::Knight)
}

fn w_b() -> Piece {
    Piece::new(Side::White, Type::Bishop)
}

fn w_p() -> Piece {
    Piece::new(Side::White, Type::Pawn)
}

fn w_k() -> Piece {
    Piece::new(Side::White, Type::King)
}

fn w_q() -> Piece {
    Piece::new(Side::White, Type::Queen)
}

fn w_r() -> Piece {
    Piece::new(Side::White, Type::Rook)
}

fn w_kn() -> Piece {
    Piece::new(Side::White, Type::Knight)
}

fn n() -> Piece {
    Piece::new(Side::None, Type::Empty)
}

pub struct Board {
    board: [Piece; 64],
}

impl Board {
    pub fn print(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{}", self.board[i * 8 + j].to_string());
            }
            print!("\n");
        }
    }

    pub fn new() -> Board {
        Board {
            board: [
                b_r(),
                b_kn(),
                b_b(),
                b_k(),
                b_q(),
                b_b(),
                b_kn(),
                b_r(),
                b_p(),
                b_p(),
                b_p(),
                b_p(),
                b_p(),
                b_p(),
                b_p(),
                b_p(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                n(),
                w_p(),
                w_p(),
                w_p(),
                w_p(),
                w_p(),
                w_p(),
                w_p(),
                w_p(),
                w_r(),
                w_kn(),
                w_b(),
                w_k(),
                w_q(),
                w_b(),
                w_kn(),
                w_r(),
            ],
        }
    }
}
