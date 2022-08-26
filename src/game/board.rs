use super::piece::Piece;

pub struct Board {
    board: [Piece; 64],
}

fn p(m: &str) -> Piece {
    Piece::new_for_str(m)
}

impl Board {
    pub fn get_piece(&mut self, pos: (usize, usize)) -> Option<&Piece> {
        let idx = Board::pos_to_idx(pos);
        if idx > 63 {
            return None;
        }
        Some(&self.board[idx])
    }

    pub fn print(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{}", self.board[(7 - i) * 8 + j].to_string());
            }
            print!("\n");
        }
    }

    pub fn set(&mut self, pos: (usize, usize), piece: Piece) {
        let idx = Board::pos_to_idx(pos);
        self.board[idx] = piece;
    }

    fn pos_to_idx(pos: (usize, usize)) -> usize {
        pos.0 + pos.1 * 8
    }

    pub fn new() -> Board {
        Board {
            board: [
                p("wr"),
                p("wkn"),
                p("wb"),
                p("wq"),
                p("wk"),
                p("wb"),
                p("wkn"),
                p("wr"),
                p("wp"),
                p("wp"),
                p("wp"),
                p("wp"),
                p("wp"),
                p("wp"),
                p("wp"),
                p("wp"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("e"),
                p("bp"),
                p("bp"),
                p("bp"),
                p("bp"),
                p("bp"),
                p("bp"),
                p("bp"),
                p("bp"),
                p("br"),
                p("bkn"),
                p("bb"),
                p("bq"),
                p("bk"),
                p("bb"),
                p("bkn"),
                p("br"),
            ],
        }
    }
}
