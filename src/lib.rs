use crate::gosettings::GoSettings;
use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub mod go;
pub mod gosettings;
pub mod listen;
pub mod moves;
pub mod options;
pub mod position;
pub mod setoption;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UGIGameResult {
    P1Win,
    P2Win,
    Draw,
}

pub trait UGI {
    fn init(&mut self);

    fn shutdown(&mut self);

    fn name(&self) -> String;

    fn author(&self) -> String;

    fn uginewgame(&mut self);

    fn isready(&mut self);

    fn position(&mut self, fen: &str);

    fn moves(&mut self, movestr: &str);

    fn go(&mut self, settings: &GoSettings);

    fn perft(&mut self, depth: u32);

    fn split(&mut self, depth: u32);

    fn stop(&mut self);

    fn print_options(&self);

    fn movelist(&self);

    fn set_option(&mut self, name: &str, value: &str);

    fn unrecognised_command(&mut self, word: &str, args: &mut Peekable<SplitAsciiWhitespace>) {
        let _ = (word, args);
    }

    #[must_use]
    fn query_p1turn(&self) -> bool;

    #[must_use]
    fn query_gameover(&self) -> bool {
        self.query_result().is_some()
    }

    #[must_use]
    fn query_result(&self) -> Option<UGIGameResult>;
}
