use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, func: impl FnMut(&str)) {
    stream.for_each(func)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(input: &str) -> Vec<String> {
        let mut stream = input.split_ascii_whitespace().peekable();
        let mut moves = Vec::new();
        parse(&mut stream, |movestr| moves.push(movestr.to_owned()));
        moves
    }

    #[test]
    fn single_move_is_parsed() {
        assert_eq!(parse_str("e2e4"), vec!["e2e4"]);
    }

    #[test]
    fn multiple_moves_are_parsed_in_order() {
        assert_eq!(parse_str("e2e4 c7c5 g1f3"), vec!["e2e4", "c7c5", "g1f3"]);
    }

    #[test]
    fn empty_input_yields_no_moves() {
        assert!(parse_str("").is_empty());
    }
}
