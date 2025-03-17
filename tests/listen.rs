use std::io::{BufReader, Read};
use std::iter::Peekable;
use std::str::SplitAsciiWhitespace;
use ugi::gosettings::GoSettings;
use ugi::listen::listen_impl;
use ugi::{UGIGameResult, UGI};

#[derive(Default)]
struct FakeState {
    received: Vec<String>,
}

impl UGI for FakeState {
    fn init(&mut self) {
        self.received.push("init".to_string());
    }

    fn shutdown(&mut self) {
        self.received.push("shutdown".to_string());
    }

    fn name(&self) -> String {
        String::new()
    }

    fn author(&self) -> String {
        String::new()
    }

    fn uginewgame(&mut self) {
        self.received.push("uginewgame".to_string());
    }

    fn isready(&mut self) {
        self.received.push("isready".to_string());
    }

    fn position(&mut self, _fen: &str) {
        self.received.push("position".to_string());
    }

    fn moves(&mut self, _movestr: &str) {
        self.received.push("moves".to_string());
    }

    fn go(&mut self, _settings: &GoSettings) {
        self.received.push("go".to_string());
    }

    fn perft(&mut self, _depth: u32) {}

    fn split(&mut self, _depth: u32) {}

    fn stop(&mut self) {}

    fn print_options(&self) {}

    fn movelist(&self) {}

    fn set_option(&mut self, name: &str, value: &str) {
        self.received.push(format!("setoption {} {}", name, value));
    }

    fn unrecognised_command(&mut self, _word: &str, _args: &mut Peekable<SplitAsciiWhitespace>) {}

    fn query_p1turn(&self) -> bool {
        true
    }

    fn query_result(&self) -> Option<UGIGameResult> {
        None
    }
}

// Input that fails if its end keeps being read, so a listener that ignores
// the end of input fails rather than looping forever. The end can be read
// twice: once to finish a last line without a newline, and once more to find
// there's nothing after it.
struct LimitedEnd<'a> {
    data: &'a [u8],
    ends_read: usize,
}

impl Read for LimitedEnd<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.data.is_empty() {
            self.ends_read += 1;
            if self.ends_read > 2 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Read past the end of input",
                ));
            }
            return Ok(0);
        }

        let n = self.data.read(buf)?;
        Ok(n)
    }
}

fn run(input: &str) -> std::io::Result<Vec<String>> {
    let mut state = FakeState::default();
    let reader = BufReader::new(LimitedEnd {
        data: input.as_bytes(),
        ends_read: 0,
    });
    listen_impl(&mut state, reader, std::io::sink())?;
    Ok(state.received)
}

#[test]
fn quit() {
    assert_eq!(run("quit\n").unwrap(), ["shutdown"]);
    assert_eq!(
        run("isready\nquit\n").unwrap(),
        ["init", "isready", "shutdown"]
    );
}

#[test]
fn end_of_input_quits() {
    for (input, expected) in [
        ("", vec!["shutdown"]),
        ("\n\n", vec!["shutdown"]),
        (
            "setoption name Hash value 64\n",
            vec!["setoption Hash 64", "shutdown"],
        ),
        ("isready\n", vec!["init", "isready", "shutdown"]),
        (
            "isready\ngo depth 1\n",
            vec!["init", "isready", "go", "shutdown"],
        ),
        // The last line doesn't need a newline
        (
            "isready\ngo depth 1",
            vec!["init", "isready", "go", "shutdown"],
        ),
    ] {
        assert_eq!(run(input).unwrap(), expected, "{input:?}");
    }
}

#[test]
fn blank_lines_are_ignored() {
    assert_eq!(
        run("\n\nisready\n\n\nquit\n").unwrap(),
        ["init", "isready", "shutdown"]
    );
}
