use std::{iter::Peekable, str::SplitAsciiWhitespace};

#[must_use]
fn collect_until(stream: &mut Peekable<SplitAsciiWhitespace<'_>>, keyword: &str) -> String {
    let mut result = String::new();

    while let Some(&token) = stream.peek() {
        if token == keyword {
            break;
        }

        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(stream.next().unwrap());
    }

    result
}

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&str, &str)) {
    if stream.next() != Some("name") {
        return;
    }

    let name = collect_until(stream, "value");

    if stream.next() != Some("value") {
        return;
    }

    let value = stream.collect::<Vec<_>>().join(" ");

    // let Some(value) = stream.next() else {
    //     return;
    // };

    func(&name, &value);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(input: &str) -> Option<(String, String)> {
        let mut stream = input.split_ascii_whitespace().peekable();
        let mut result = None;
        parse(&mut stream, |name, value| {
            result = Some((name.to_owned(), value.to_owned()));
        });
        result
    }

    #[test]
    fn valid() {
        let pairs = [
            ("a", "1"),
            ("a b", "1"),
            ("a", "1 2"),
            ("a b", "1 2"),
            ("abc def", "123 456"),
            ("a b c d e f", "1 2 3 4 5 6"),
        ];

        for (name, value) in pairs {
            let input = format!("name {} value {}", name, value);
            let Some((a, b)) = parse_str(&input) else {
                panic!("Failed on: {}", input);
            };
            assert_eq!(name, a, "Parse name failed: {}", input);
            assert_eq!(value, b, "Parse value failed: {}", input);
        }
    }

    #[test]
    fn missing_keywords() {
        assert_eq!(parse_str("Hash value 1000"), None);
        assert_eq!(parse_str("name Hash 1000"), None);
    }

    #[test]
    fn invalid() {
        let pairs = [
            ("a", "1"),
            ("a b", "1"),
            ("a", "1 2"),
            ("a b", "1 2"),
            ("abc def", "123 456"),
            ("a b c d e f", "1 2 3 4 5 6"),
        ];

        for (name, value) in pairs {
            let input = format!("name {} value {}", name, value);
            let Some((a, b)) = parse_str(&input) else {
                panic!("Failed on: {}", input);
            };
            assert_eq!(name, a, "Parse name failed: {}", input);
            assert_eq!(value, b, "Parse value failed: {}", input);
        }
    }
}
