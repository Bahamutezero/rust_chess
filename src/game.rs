use regex::Regex;
use std::io;
use std::io::Write;

pub mod board;
pub mod piece;

use board::Board;
use piece::Piece;
use piece::Side;
use piece::Type;

pub struct Game {
    pub state: State,
    pub side: Side,
    pub board: Board,
}

#[derive(PartialEq)]
pub enum State {
    Started,
    NotStarted,
    GameOver,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::NotStarted,
            side: Side::White,
            board: Board::new(),
        }
    }

    pub fn start_game_loop(&mut self) {
        loop {
            let cmd = self.get_input();
            self.handle_command(cmd)
        }
    }

    fn get_indices(&mut self, coord: &str) -> (usize, usize) {
        let mut iter = coord.chars();
        let x = self.map_indice(iter.nth(0));
        let y = self.map_indice(iter.nth(0));
        return (x, y);
    }

    fn valid_position(&mut self, coord: &str) -> bool {
        let re = Regex::new(r"[a-h][1-8]").unwrap();
        let matched = re.is_match(coord);
        if !matched {
            println!(
                "Invalid position: Please make sure your positions are in the form: [a-h][1-8]"
            );
            return false;
        }
        matched
    }

    fn map_indice(&mut self, c: Option<char>) -> usize {
        match c {
            Some('a') | Some('1') => 0,
            Some('b') | Some('2') => 1,
            Some('c') | Some('3') => 2,
            Some('d') | Some('4') => 3,
            Some('e') | Some('5') => 4,
            Some('f') | Some('6') => 5,
            Some('g') | Some('7') => 6,
            Some('h') | Some('8') => 7,
            Some(_) => panic!("Value must be the supported above."),
            None => panic!("There should definitely be a value."),
        }
    }

    fn move_piece(&mut self, start: &str, end: &str) {
        let start = start.to_ascii_lowercase();
        let end = end.to_ascii_lowercase();

        if !self.valid_position(&start) || !self.valid_position(&end) {
            return;
        }

        let start_pos = self.get_indices(&start);
        let end_pos = self.get_indices(&end);

        if !self.valid_move(start_pos, end_pos) {
            println!("invalid move");
            return;
        }
        let piece = self.board.get_piece(start_pos);
        if let Some(piece) = piece {
            let cloned = Piece::clone(piece);
            self.board.set(end_pos, cloned);
        }
        self.board.set(start_pos, Piece::empty());
        self.switch_side();
    }

    fn switch_side(&mut self) {
        self.side = if self.side == Side::Black {
            Side::White
        } else {
            Side::Black
        };
        self.declare_side();
    }

    fn declare_side(&mut self) {
        if self.state == State::Started {
            let side = if self.side == Side::White {
                "white's"
            } else {
                "black's"
            };
            println!("{} move", side)
        }
    }

    fn valid_move(&mut self, start_pos: (usize, usize), end_pos: (usize, usize)) -> bool {
        let piece = self.board.get_piece(start_pos);
        if let Some(piece) = piece {
            if piece.side != self.side {
                return false;
            }
        }
        let possible_moves = self.get_all_possible_moves(start_pos);
        possible_moves.contains(&end_pos)
    }

    fn get_all_possible_moves(&mut self, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let piece = self.board.get_piece(start_pos);
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        if let Some(piece) = piece {
            let piece = Piece::clone(piece);
            possible_moves = match piece.piece_type {
                Type::King => self.king_moves(start_pos),
                Type::Pawn => self.pawn_moves(piece.side, start_pos),
                Type::Queen => [self.bishop_moves(start_pos), self.rook_moves(start_pos)].concat(),
                Type::Knight => self.knight_moves(start_pos),
                Type::Rook => self.rook_moves(start_pos),
                Type::Bishop => self.bishop_moves(start_pos),
                Type::Empty => panic!("No empty possible moves"),
            }
        }
        possible_moves
    }

    fn king_moves(&mut self, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        let mut check_existence = |pos: (usize, usize)| -> bool {
            let piece = self.board.get_piece(pos);
            if let Some(piece) = piece {
                if piece.side == Side::None {
                    possible_moves.push(pos);
                    return true;
                }
                if piece.side != self.side {
                    possible_moves.push(pos);
                }
            }
            false
        };
        let x = start_pos.0;
        let y = start_pos.1;
        check_existence((x + 1, y + 1));
        check_existence((x + 1, y));
        check_existence((x, y + 1));
        if x > 0 {
            check_existence((x - 1, y + 1));
            check_existence((x - 1, y));
        }

        if y > 0 {
            check_existence((x, y - 1));
            check_existence((x + 1, y - 1));
        }

        if x > 0 && y > 0 {
            check_existence((x - 1, y - 1));
        }
        possible_moves
    }

    fn rook_moves(&mut self, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        let mut check_existence = |pos: (usize, usize)| -> bool {
            let piece = self.board.get_piece(pos);
            if let Some(piece) = piece {
                if piece.side == Side::None {
                    possible_moves.push(pos);
                    return true;
                }
                if piece.side != self.side {
                    possible_moves.push(pos);
                }
            }
            false
        };
        let mut x = start_pos.0 + 1;
        let y = start_pos.1;
        while check_existence((x, y)) {
            x += 1;
        }

        if start_pos.0 > 0 {
            let mut x = start_pos.0 - 1;
            while check_existence((x, y)) && x != 0 {
                x -= 1;
            }
        }

        let x = start_pos.0;
        let mut y = start_pos.1 + 1;
        while check_existence((x, y)) {
            y += 1;
        }

        if start_pos.1 > 0 {
            let mut y = start_pos.1 - 1;
            while check_existence((x, y)) && y != 0 {
                y -= 1;
            }
        }
        possible_moves
    }

    fn bishop_moves(&mut self, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        let mut check_existence = |pos: (usize, usize)| -> bool {
            let piece = self.board.get_piece(pos);
            if let Some(piece) = piece {
                if piece.side == Side::None {
                    possible_moves.push(pos);
                    return true;
                }
                if piece.side != self.side {
                    possible_moves.push(pos);
                }
            }
            false
        };
        let mut x = start_pos.0 + 1;
        let mut y = start_pos.1 + 1;
        while check_existence((x, y)) {
            x += 1;
            y += 1;
        }

        if start_pos.0 > 0 {
            let mut x = start_pos.0 - 1;
            let mut y = start_pos.1 + 1;
            while check_existence((x, y)) && x != 0 {
                x -= 1;
                y += 1;
            }
        }

        if start_pos.1 > 0 {
            let mut x = start_pos.0 + 1;
            let mut y = start_pos.1 - 1;
            while check_existence((x, y)) && y != 0 {
                x += 1;
                y -= 1;
            }
        }

        if start_pos.0 > 0 && start_pos.1 > 0 {
            let mut x = start_pos.0 - 1;
            let mut y = start_pos.1 - 1;
            while check_existence((x, y)) && (y != 0 && x != 0) {
                x -= 1;
                y -= 1;
            }
        }
        possible_moves
    }

    fn knight_moves(&mut self, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        let mut check_existence = |pos: (usize, usize)| {
            let piece = self.board.get_piece(pos);
            if let Some(piece) = piece {
                if piece.side == Side::None || (piece.side != self.side) {
                    possible_moves.push(pos);
                }
            }
        };
        check_existence((start_pos.0 + 1, start_pos.1 + 2));
        check_existence((start_pos.0 + 2, start_pos.1 + 1));
        if start_pos.1 > 0 {
            check_existence((start_pos.0 + 2, start_pos.1 - 1));
        }
        if start_pos.1 > 1 {
            check_existence((start_pos.0 + 1, start_pos.1 - 2));
        }

        if start_pos.0 > 0 && start_pos.1 > 1 {
            check_existence((start_pos.0 - 1, start_pos.1 - 2));
        }

        if start_pos.0 > 1 && start_pos.1 > 0 {
            check_existence((start_pos.0 - 2, start_pos.1 - 1));
        }

        if start_pos.0 > 1 {
            check_existence((start_pos.0 - 2, start_pos.1 + 1));
        }

        if start_pos.0 > 0 {
            check_existence((start_pos.0 - 1, start_pos.1 + 2));
        }
        possible_moves
    }

    fn pawn_moves(&mut self, side: Side, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();

        let mut check_existence = |pos: (usize, usize), should_exist: bool| {
            let piece = self.board.get_piece(pos);
            if let Some(piece) = piece {
                if should_exist && piece.side != Side::None && piece.side != self.side {
                    possible_moves.push(pos);
                }

                if !should_exist && piece.side == Side::None {
                    possible_moves.push(pos);
                }
            }
        };

        let mut y_coord = start_pos.1 + 1;
        if side == Side::Black {
            y_coord = start_pos.1 - 1;
        }

        check_existence((start_pos.0, y_coord), false);

        if side == Side::White && start_pos.1 == 1 {
            check_existence((start_pos.0, start_pos.1 + 2), false);
        }

        if side == Side::Black && start_pos.1 == 6 {
            check_existence((start_pos.0, start_pos.1 - 2), false);
        }

        if start_pos.0 != 0 {
            check_existence((start_pos.0 - 1, y_coord), true);
        }
        check_existence((start_pos.1 + 1, y_coord), true);

        possible_moves
    }

    fn prompt(&mut self) {
        print!(">> ");
        io::stdout().flush().expect("unable to flush");
    }

    fn get_input(&mut self) -> String {
        self.prompt();
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("failed to read line");
        return inp.trim_end().to_string();
    }

    fn handle_command(&mut self, inp: String) {
        let v: Vec<&str> = inp.split(' ').collect();
        let cmd = v[0];
        match cmd {
            "s" | "start" => self.start(),
            "b" | "board" => self.board(),
            "m" | "move" => {
                if v.len() < 3 {
                    println!("Please specify start position and an end position.");
                    println!("For Example: move a1 a8");
                    return;
                }
                let start = v[1];
                let end = v[2];
                self.move_piece(start, end);
            }
            "p" | "piece" => {
                if v.len() < 2 {
                    println!("Please specify a position to look up.");
                    println!("For Example: piece a1");
                    return;
                }
                let coord = v[1];
                self.piece_at(coord)
            }
            &_ => return,
        }
    }

    fn piece_at(&mut self, coord: &str) {
        let coord = coord.to_ascii_lowercase();
        if !self.valid_position(&coord) {
            return;
        }
        let pos = self.get_indices(&coord);
        let piece = self.board.get_piece(pos);
        if let Some(piece) = piece {
            println!("{}", piece.to_string())
        }
    }

    fn board(&mut self) {
        if self.state == State::NotStarted || self.state == State::GameOver {
            println!("No active game");
            return;
        }
        self.board.print()
    }

    fn start(&mut self) {
        if self.state == State::Started {
            println!("There's an ongoing game.");
            return;
        }
        self.state = State::Started;
        self.declare_side();
    }
}
