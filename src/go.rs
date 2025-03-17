use crate::GoSettings;
use std::{
    fmt,
    iter::Peekable,
    str::{FromStr, SplitAsciiWhitespace},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoParseError {
    MissingValue {
        keyword: &'static str,
    },
    InvalidNumber {
        keyword: &'static str,
        value: String,
    },
    OutOfRange {
        keyword: &'static str,
        value: String,
        constraint: &'static str,
    },
    DuplicateKeyword {
        keyword: &'static str,
    },
    UnrecognisedKeyword {
        keyword: String,
    },
    NoSearchLimit,
}

impl fmt::Display for GoParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingValue { keyword } => {
                write!(f, "'{keyword}' is missing its value")
            }
            Self::InvalidNumber { keyword, value } => {
                write!(f, "'{keyword}' value '{value}' is not a valid number")
            }
            Self::OutOfRange {
                keyword,
                value,
                constraint,
            } => {
                write!(f, "'{keyword}' value '{value}' must be {constraint}")
            }
            Self::DuplicateKeyword { keyword } => {
                write!(f, "'{keyword}' was specified more than once")
            }
            Self::UnrecognisedKeyword { keyword } => {
                write!(f, "unrecognised keyword '{keyword}'")
            }
            Self::NoSearchLimit => write!(
                f,
                "no search limit given, expected at least one of p1time, p2time, depth, nodes, movetime, or infinite"
            ),
        }
    }
}

impl std::error::Error for GoParseError {}

#[must_use]
fn show_or<T>(value: Option<T>) -> String
where
    T: ToString,
{
    if let Some(n) = value {
        n.to_string()
    } else {
        "None".to_string()
    }
}

#[must_use]
pub fn to_ugi_string(settings: &GoSettings) -> String {
    format!(
        "go p1time {} p2time {} p1inc {} p2inc {} depth {} nodes {} movetime {} movestogo {} infinite {}",
        show_or(settings.p1time),
        show_or(settings.p2time),
        show_or(settings.p1inc),
        show_or(settings.p2inc),
        show_or(settings.depth),
        show_or(settings.nodes),
        show_or(settings.movetime),
        show_or(settings.movestogo),
        settings.infinite,
    )
}

fn parse_number<T: FromStr>(
    stream: &mut Peekable<SplitAsciiWhitespace>,
    keyword: &'static str,
) -> Result<T, GoParseError> {
    let Some(word) = stream.next() else {
        return Err(GoParseError::MissingValue { keyword });
    };

    word.parse::<T>().map_err(|_| GoParseError::InvalidNumber {
        keyword,
        value: word.to_owned(),
    })
}

fn parse_arg<T: FromStr + PartialOrd + ToString + Copy>(
    stream: &mut Peekable<SplitAsciiWhitespace>,
    keyword: &'static str,
    min: T,
    constraint: &'static str,
    slot: &mut Option<T>,
) -> Result<(), GoParseError> {
    let value = parse_number::<T>(stream, keyword)?;

    if value < min {
        return Err(GoParseError::OutOfRange {
            keyword,
            value: value.to_string(),
            constraint,
        });
    }

    if slot.is_some() {
        return Err(GoParseError::DuplicateKeyword { keyword });
    }

    *slot = Some(value);

    Ok(())
}

pub(crate) fn parse_depth(
    stream: &mut Peekable<SplitAsciiWhitespace>,
) -> Result<i32, GoParseError> {
    let mut depth = None;
    parse_arg(stream, "depth", 1, "greater than 0", &mut depth)?;
    Ok(depth.expect("parse_arg sets Some on success"))
}

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>) -> Result<GoSettings, GoParseError> {
    let mut settings = GoSettings::default();

    while let Some(word) = stream.next() {
        match word {
            // Single
            "infinite" => settings.infinite = true,
            // Double
            "p1time" => parse_arg(stream, "p1time", 1, "greater than 0", &mut settings.p1time)?,
            "p2time" => parse_arg(stream, "p2time", 1, "greater than 0", &mut settings.p2time)?,
            "p1inc" => parse_arg(
                stream,
                "p1inc",
                0,
                "greater than or equal to 0",
                &mut settings.p1inc,
            )?,
            "p2inc" => parse_arg(
                stream,
                "p2inc",
                0,
                "greater than or equal to 0",
                &mut settings.p2inc,
            )?,
            "depth" => parse_arg(
                stream,
                "depth",
                0,
                "greater than or equal to 0",
                &mut settings.depth,
            )?,
            "nodes" => parse_arg(stream, "nodes", 1, "greater than 0", &mut settings.nodes)?,
            "movetime" => parse_arg(
                stream,
                "movetime",
                1,
                "greater than 0",
                &mut settings.movetime,
            )?,
            "movestogo" => parse_arg(
                stream,
                "movestogo",
                1,
                "greater than 0",
                &mut settings.movestogo,
            )?,
            keyword => {
                return Err(GoParseError::UnrecognisedKeyword {
                    keyword: keyword.to_owned(),
                });
            }
        }
    }

    if settings.p1time.is_none()
        && settings.p2time.is_none()
        && settings.depth.is_none()
        && settings.nodes.is_none()
        && settings.movetime.is_none()
        && !settings.infinite
    {
        return Err(GoParseError::NoSearchLimit);
    }

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(input: &str) -> Result<GoSettings, GoParseError> {
        let mut stream = input.split_ascii_whitespace().peekable();
        parse(&mut stream)
    }

    #[test]
    fn empty_input_has_no_search_limit() {
        assert_eq!(parse_str(""), Err(GoParseError::NoSearchLimit));
    }

    #[test]
    fn infinite_alone_is_valid() {
        let settings = parse_str("infinite").unwrap();
        assert!(settings.infinite);
    }

    #[test]
    fn movetime_is_parsed() {
        let settings = parse_str("movetime 500").unwrap();
        assert_eq!(settings.movetime, Some(500));
    }

    #[test]
    fn depth_zero_is_allowed() {
        let settings = parse_str("depth 0").unwrap();
        assert_eq!(settings.depth, Some(0));
    }

    #[test]
    fn depth_negative_is_out_of_range() {
        assert_eq!(
            parse_str("depth -1"),
            Err(GoParseError::OutOfRange {
                keyword: "depth",
                value: "-1".to_owned(),
                constraint: "greater than or equal to 0",
            })
        );
    }

    #[test]
    fn nodes_zero_is_out_of_range() {
        assert_eq!(
            parse_str("nodes 0"),
            Err(GoParseError::OutOfRange {
                keyword: "nodes",
                value: "0".to_owned(),
                constraint: "greater than 0",
            })
        );
    }

    #[test]
    fn p1time_zero_is_out_of_range() {
        assert_eq!(
            parse_str("p1time 0"),
            Err(GoParseError::OutOfRange {
                keyword: "p1time",
                value: "0".to_owned(),
                constraint: "greater than 0",
            })
        );
    }

    #[test]
    fn p1inc_zero_is_allowed() {
        let settings = parse_str("p1time 1000 p1inc 0").unwrap();
        assert_eq!(settings.p1inc, Some(0));
    }

    #[test]
    fn full_time_control_is_parsed() {
        let settings = parse_str("p1time 1000 p2time 2000 p1inc 10 p2inc 20").unwrap();
        assert_eq!(settings.p1time, Some(1000));
        assert_eq!(settings.p2time, Some(2000));
        assert_eq!(settings.p1inc, Some(10));
        assert_eq!(settings.p2inc, Some(20));
    }

    #[test]
    fn duplicate_keyword_is_rejected() {
        assert_eq!(
            parse_str("movetime 500 movetime 600"),
            Err(GoParseError::DuplicateKeyword {
                keyword: "movetime"
            })
        );
    }

    #[test]
    fn missing_value_is_rejected() {
        assert_eq!(
            parse_str("movetime"),
            Err(GoParseError::MissingValue {
                keyword: "movetime"
            })
        );
    }

    #[test]
    fn non_numeric_value_is_rejected() {
        assert_eq!(
            parse_str("movetime abc"),
            Err(GoParseError::InvalidNumber {
                keyword: "movetime",
                value: "abc".to_owned(),
            })
        );
    }

    #[test]
    fn unrecognised_keyword_is_rejected() {
        assert_eq!(
            parse_str("banana"),
            Err(GoParseError::UnrecognisedKeyword {
                keyword: "banana".to_owned(),
            })
        );
    }

    #[test]
    fn perft_and_split_are_unrecognised_keywords() {
        assert_eq!(
            parse_str("perft 5"),
            Err(GoParseError::UnrecognisedKeyword {
                keyword: "perft".to_owned(),
            })
        );
        assert_eq!(
            parse_str("split 3"),
            Err(GoParseError::UnrecognisedKeyword {
                keyword: "split".to_owned(),
            })
        );
    }

    #[test]
    fn to_ugi_string_reflects_settings() {
        let settings = GoSettings {
            movetime: Some(500),
            ..GoSettings::default()
        };
        assert_eq!(
            to_ugi_string(&settings),
            "go p1time None p2time None p1inc None p2inc None depth None nodes None movetime 500 movestogo None infinite false"
        );
    }

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(
            GoParseError::MissingValue { keyword: "depth" }.to_string(),
            "'depth' is missing its value"
        );
        assert_eq!(
            GoParseError::InvalidNumber {
                keyword: "depth",
                value: "abc".to_owned(),
            }
            .to_string(),
            "'depth' value 'abc' is not a valid number"
        );
        assert_eq!(
            GoParseError::OutOfRange {
                keyword: "depth",
                value: "-1".to_owned(),
                constraint: "greater than or equal to 0",
            }
            .to_string(),
            "'depth' value '-1' must be greater than or equal to 0"
        );
        assert_eq!(
            GoParseError::DuplicateKeyword { keyword: "depth" }.to_string(),
            "'depth' was specified more than once"
        );
        assert_eq!(
            GoParseError::UnrecognisedKeyword {
                keyword: "banana".to_owned(),
            }
            .to_string(),
            "unrecognised keyword 'banana'"
        );
        assert_eq!(
            GoParseError::NoSearchLimit.to_string(),
            "no search limit given, expected at least one of p1time, p2time, depth, nodes, movetime, or infinite"
        );
    }
}
