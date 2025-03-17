use std::{fmt, iter::Peekable, str::SplitAsciiWhitespace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionParseError {
    MissingKeyword,
    UnrecognisedKeyword(String),
    EmptyFen,
}

impl fmt::Display for PositionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingKeyword => write!(f, "expected 'startpos' or 'fen <fen>'"),
            Self::UnrecognisedKeyword(keyword) => {
                write!(f, "expected 'startpos' or 'fen', got '{keyword}'")
            }
            Self::EmptyFen => write!(f, "'fen' keyword given but no FEN string followed"),
        }
    }
}

impl std::error::Error for PositionParseError {}

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>) -> Result<String, PositionParseError> {
    match stream.next() {
        Some("startpos") => Ok("startpos".to_owned()),
        Some("fen") => {
            let mut fen = String::new();
            while let Some(word) = stream.peek() {
                if *word == "moves" {
                    break;
                }

                if !fen.is_empty() {
                    fen += " ";
                }
                fen += word;

                stream.next();
            }

            if fen.is_empty() {
                Err(PositionParseError::EmptyFen)
            } else {
                Ok(fen)
            }
        }
        Some(keyword) => Err(PositionParseError::UnrecognisedKeyword(keyword.to_owned())),
        None => Err(PositionParseError::MissingKeyword),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(input: &str) -> Result<String, PositionParseError> {
        let mut stream = input.split_ascii_whitespace().peekable();
        parse(&mut stream)
    }

    #[test]
    fn startpos_is_parsed() {
        assert_eq!(parse_str("startpos"), Ok("startpos".to_owned()));
    }

    #[test]
    fn fen_is_parsed() {
        assert_eq!(
            parse_str("fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            Ok("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned())
        );
    }

    #[test]
    fn fen_stops_at_moves() {
        assert_eq!(
            parse_str("fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4"),
            Ok("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned())
        );
    }

    #[test]
    fn empty_fen_is_rejected() {
        assert_eq!(parse_str("fen"), Err(PositionParseError::EmptyFen));
    }

    #[test]
    fn missing_keyword_is_rejected() {
        assert_eq!(parse_str(""), Err(PositionParseError::MissingKeyword));
    }

    #[test]
    fn unrecognised_keyword_is_rejected() {
        assert_eq!(
            parse_str("banana"),
            Err(PositionParseError::UnrecognisedKeyword("banana".to_owned()))
        );
    }

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(
            PositionParseError::MissingKeyword.to_string(),
            "expected 'startpos' or 'fen <fen>'"
        );
        assert_eq!(
            PositionParseError::UnrecognisedKeyword("banana".to_owned()).to_string(),
            "expected 'startpos' or 'fen', got 'banana'"
        );
        assert_eq!(
            PositionParseError::EmptyFen.to_string(),
            "'fen' keyword given but no FEN string followed"
        );
    }
}
