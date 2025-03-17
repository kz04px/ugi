use std::iter::Peekable;
use std::str::SplitAsciiWhitespace;
use ugi::{gosettings::GoSettings, listen, UGIGameResult, UGI};

const SIZE: usize = 3;
const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Piece {
    X,
    O,
}

#[derive(Clone, Copy)]
struct TicTacToe {
    board: [Option<Piece>; 9],
    x_to_move: bool,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            board: [None; 9],
            x_to_move: true,
        }
    }

    fn set_position(&mut self, fen: &str) {
        *self = Self::new();

        if fen == "startpos" {
            return;
        }

        let mut parts = fen.split_ascii_whitespace();
        if let Some(cells) = parts.next() {
            for (index, c) in cells.chars().take(9).enumerate() {
                self.board[index] = match c {
                    'x' => Some(Piece::X),
                    'o' => Some(Piece::O),
                    _ => None,
                };
            }
        }
        self.x_to_move = parts.next() != Some("o");
    }

    fn square_index(square: &str) -> Option<usize> {
        let bytes = square.as_bytes();
        if bytes.len() != 2 {
            return None;
        }

        let col = usize::from(bytes[0].checked_sub(b'a')?);
        let row = usize::from(bytes[1].checked_sub(b'1')?);
        if col >= SIZE || row >= SIZE {
            return None;
        }

        Some((SIZE - 1 - row) * SIZE + col)
    }

    fn square_name(index: usize) -> String {
        let col = index % SIZE;
        let row = SIZE - 1 - index / SIZE;
        format!("{}{}", (b'a' + col as u8) as char, row + 1)
    }

    fn make(&mut self, index: usize) {
        self.board[index] = if self.x_to_move {
            Some(Piece::X)
        } else {
            Some(Piece::O)
        };
        self.x_to_move = !self.x_to_move;
    }

    fn apply(&mut self, movestr: &str) {
        if let Some(index) = Self::square_index(movestr) {
            if self.board[index] == None {
                self.make(index);
            }
        }
    }

    fn legal_moves(&self) -> Vec<usize> {
        (0..9).filter(|&i| self.board[i].is_none()).collect()
    }

    fn winner(&self) -> Option<Piece> {
        LINES
            .iter()
            .map(|&[a, b, c]| (self.board[a], self.board[b], self.board[c]))
            .find(|&(a, b, c)| a.is_some() && a == b && b == c)
            .map(|(a, _, _)| a)?
    }

    fn result(&self) -> Option<UGIGameResult> {
        match self.winner() {
            Some(Piece::X) => Some(UGIGameResult::P1Win),
            Some(Piece::O) => Some(UGIGameResult::P2Win),
            None if self.legal_moves().is_empty() => Some(UGIGameResult::Draw),
            None => None,
        }
    }

    fn perft(&mut self, depth: i32) -> u64 {
        if depth == 0 || self.result().is_some() {
            return 1;
        }

        self.legal_moves()
            .into_iter()
            .map(|mv| {
                let saved = *self;
                self.make(mv);
                let nodes = self.perft(depth - 1);
                *self = saved;
                nodes
            })
            .sum()
    }

    fn print_board(&self) {
        for row in 0..SIZE {
            let cells = (0..SIZE)
                .map(|col| match self.board[row * SIZE + col] {
                    None => '.',
                    Some(Piece::X) => 'x',
                    Some(Piece::O) => 'o',
                })
                .collect::<String>();
            println!("{cells}");
        }
    }
}

impl UGI for TicTacToe {
    fn init(&mut self) {}

    fn shutdown(&mut self) {}

    fn name(&self) -> String {
        "Tic Tac Toe".to_owned()
    }

    fn author(&self) -> String {
        "UGI examples".to_owned()
    }

    fn uginewgame(&mut self) {
        *self = Self::new();
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn position(&mut self, fen: &str) {
        self.set_position(fen);
    }

    fn moves(&mut self, movestr: &str) {
        self.apply(movestr);
    }

    fn go(&mut self, _settings: &GoSettings) {
        let best = self
            .legal_moves()
            .into_iter()
            .next()
            .expect("go() is never called on a position without legal moves");

        println!("bestmove {}", Self::square_name(best));
    }

    fn perft(&mut self, settings: &GoSettings) {
        let depth = settings.depth.unwrap_or(0);
        println!("nodes {}", self.perft(depth));
    }

    fn split(&mut self, settings: &GoSettings) {
        let depth = settings.depth.unwrap_or(0);
        let mut total = 0;
        for mv in self.legal_moves() {
            let saved = *self;
            self.make(mv);
            let nodes = self.perft(depth.saturating_sub(1));
            *self = saved;
            total += nodes;
            println!("{}: {}", Self::square_name(mv), nodes);
        }
        println!("nodes {total}");
    }

    fn stop(&mut self) {}

    fn print_options(&self) {}

    fn movelist(&self) {
        let moves = self
            .legal_moves()
            .into_iter()
            .map(Self::square_name)
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", moves);
    }

    fn set_option(&mut self, _name: &str, _value: &str) {}

    fn query_p1turn(&self) -> bool {
        self.x_to_move
    }

    fn query_result(&self) -> Option<UGIGameResult> {
        self.result()
    }

    fn unrecognised_command(&mut self, word: &str, _args: &mut Peekable<SplitAsciiWhitespace>) {
        if word == "board" {
            self.print_board();
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut first_line = String::new();
    std::io::stdin().read_line(&mut first_line)?;

    match first_line.trim() {
        "ugi" => {
            let mut engine = TicTacToe::new();
            listen::listen(&mut engine)
        }
        _ => Ok(()),
    }
}
