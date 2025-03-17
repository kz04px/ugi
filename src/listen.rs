use crate::go;
use crate::moves;
use crate::position;
use crate::setoption;
use crate::UGIGameResult;
use crate::UGI;
use std::io::BufRead;
use std::io::Write;

pub fn listen(state: &mut dyn UGI) -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    listen_impl(state, stdin.lock(), stdout.lock())
}

pub fn listen_impl(
    state: &mut dyn UGI,
    mut reader: impl BufRead,
    mut write_output: impl Write,
) -> std::io::Result<()> {
    writeln!(write_output, "id name {}", state.name())?;
    writeln!(write_output, "id author {}", state.author())?;
    state.print_options();
    writeln!(write_output, "ugiok")?;

    let mut input = String::new();

    // Pre isready setup
    loop {
        // End of input is the controller having gone away
        if reader.read_line(&mut input)? == 0 {
            state.shutdown();
            return Ok(());
        }

        let mut stream = input.split_ascii_whitespace().peekable();
        match stream.next().unwrap_or("") {
            "setoption" => setoption::parse(&mut stream, |name, value| {
                state.set_option(name, value);
            }),
            "quit" => {
                state.shutdown();
                return Ok(());
            }
            "uginewgame" | "isready" | "options" | "stop" | "position" | "moves" | "go"
            | "perft" | "split" | "query" | "movelist" => {
                break;
            }
            "" => {}
            word => state.unrecognised_command(word, &mut stream),
        }

        input.clear();
    }

    state.init();

    // Post isready
    loop {
        let mut stream = input.split_ascii_whitespace().peekable();
        let mut quit = false;

        while let Some(word) = stream.next() {
            match word {
                // Singles
                "uginewgame" => state.uginewgame(),
                "isready" => state.isready(),
                "options" => state.print_options(),
                "stop" => state.stop(),
                "quit" => {
                    quit = true;
                    break;
                }
                // Multiples
                "setoption" => setoption::parse(&mut stream, |name, value| {
                    state.set_option(name, value);
                }),
                "position" => match position::parse(&mut stream) {
                    Ok(fen) => {
                        state.position(&fen);
                    }
                    Err(e) => writeln!(write_output, "info string position error {}", e)?,
                },
                "moves" => {
                    moves::parse(&mut stream, |movestr| {
                        state.moves(movestr);
                    });
                }
                "go" => match go::parse(&mut stream) {
                    Ok(n) => {
                        state.go(&n);
                    }
                    Err(e) => writeln!(write_output, "info string go error {}", e)?,
                },
                "perft" => {
                    let Some(depth) = stream.next() else {
                        break;
                    };

                    let Ok(asd) = depth.parse::<u32>() else {
                        break;
                    };

                    state.perft(asd);
                }
                "split" => {
                    let Some(depth) = stream.next() else {
                        break;
                    };

                    let Ok(asd) = depth.parse::<u32>() else {
                        break;
                    };

                    state.split(asd);
                }
                "query" => {
                    match stream.next() {
                        Some("p1turn") => {
                            writeln!(write_output, "response {}", state.query_p1turn())?
                        }
                        Some("gameover") => {
                            writeln!(write_output, "response {}", state.query_gameover())?
                        }
                        Some("result") => match state.query_result() {
                            Some(UGIGameResult::P1Win) => writeln!(write_output, "response p1win")?,
                            Some(UGIGameResult::P2Win) => writeln!(write_output, "response p2win")?,
                            Some(UGIGameResult::Draw) => writeln!(write_output, "response draw")?,
                            None => writeln!(write_output, "response none")?,
                        },
                        _ => {}
                    };
                }
                "movelist" => state.movelist(),
                word => state.unrecognised_command(word, &mut stream),
            }
        }

        if quit {
            break;
        }

        input.clear();
        if reader.read_line(&mut input)? == 0 {
            break;
        }
    }

    state.shutdown();

    Ok(())
}
