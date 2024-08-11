use crate::syntax_tree::Literal;
use nom::{
    branch::alt,
    character::complete::i32,
    combinator::{map, map_res},
    IResult,
};

/// Represents a type that can be parsed from a string
pub trait Parsable: Sized {
    /// Parses an instance of the calling type from a string. Returns the
    /// unparsed remainder of the input string and the parsed instance.
    fn parse(input: &str) -> IResult<&str, Self>;
}

impl Literal {
    fn parse_int(input: &str) -> IResult<&str, Self> {
        map(i32, |num| Literal::IntegerLiteral(num))(input)
    }
    fn parse_float(input: &str) -> IResult<&str, Self> {
        todo!()
    }
    fn parse_time(input: &str) -> IResult<&str, Self> {
        todo!()
    }
    fn parse_string(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for Literal {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            Literal::parse_int,
            Literal::parse_float,
            Literal::parse_time,
            Literal::parse_string,
        ))(input)
    }
}

#[cfg(test)]
mod literal_test {
    use super::*;

    #[test]
    fn parse_integer_literal_positive() {
        assert_eq!(
            Literal::parse("32"),
            IResult::Ok(("", Literal::IntegerLiteral(32)))
        );
    }

    #[test]
    fn parse_integer_literal_negative() {
        assert_eq!(
            Literal::parse("-32"),
            IResult::Ok(("", Literal::IntegerLiteral(-32)))
        );
    }
}
