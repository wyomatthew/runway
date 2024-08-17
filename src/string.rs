use nom::{
    bytes::complete::{is_not, tag}, character::complete::multispace1, combinator::map, sequence::delimited,
    IResult,
};

use crate::syntax_tree::Literal;

fn parse_string_literal(input: &str) -> IResult<&str, Literal> {
    let deli = delimited(tag("\""), is_not("\""), tag("\""));

    map(deli, |str_literal: &str| {
        Literal::StringLiteral(String::from(str_literal))
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_abc() {
        let data = "\"abc\"";
        let result = parse_string_literal(data);
        assert_eq!(
            result,
            Ok(("", Literal::StringLiteral(String::from("abc"))))
        );
    }
    #[test]
    fn with_whitespace() {
        let data = "\"Hello, this is a sentence.\"";
        let result = parse_string_literal(data);
        assert_eq!(
            result,
            Ok(("", Literal::StringLiteral(String::from("Hello, this is a sentence."))))
        );
    }
    #[test]
    fn with_digits() {
        let data = "\"2024\"";
        let result = parse_string_literal(data);
        assert_eq!(
            result,
            Ok(("", Literal::StringLiteral(String::from("2024"))))
        );
    }
    #[test]
    fn all_together_now() {
        let data = "\"Hello, this is a longer sentence that has numbers in it. 2024\"";
        let result = parse_string_literal(data);
        assert_eq!(
            result,
            Ok(("", Literal::StringLiteral(String::from("Hello, this is a longer sentence that has numbers in it. 2024"))))
        );
    }
    #[test]
    fn escape_me() {
        let data = "\"Hello, \\\"Here's an escaped Sentence\\\"\"";
        let result = parse_string_literal(data);
        assert_eq!(
            result,
            Ok(("", Literal::StringLiteral(String::from("Hello, \"Here's an escaped Sentence\""))))
        );
    }
}
