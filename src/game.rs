use std::io;
use std::io::Write;

pub mod board;
pub mod piece;

use board::Board;
use piece::Side;

pub struct Game {
    pub state: State,
    pub side: Side,
    pub board: Board,
    pub side_move: Side,
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
            side_move: Side::White,
        }
    }

    pub fn start_game_loop(&mut self) {
        loop {
            let cmd = self.get_input();
            self.handle_command(cmd)
        }
    }

    fn get_indices(&mut self, coord: &str) {
        coord.make_ascii_lowercase();
        let x = coord.chars().nth(0);
        let y = coord.chars().nth(1);
        println!("{}, {}", x, y)
    }

    fn move_piece(&mut self, start: &str, end: &str) {
        self.get_indices(start);
        self.get_indices(end);

        if self.state == State::Started {
            let side = if self.side_move == Side::White {
                "white's"
            } else {
                "black's"
            };
            println!("{} move", side)
        }
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
                if v.len() != 3 {
                    println!("Please specify start position and an end position.");
                    println!("For Example: move a1 a8");
                }
                let start = v[1];
                let end = v[2];
                self.move_piece(start, end);
            }
            &_ => return,
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
        println!("white's move")
    }
}
