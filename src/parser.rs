use crate::syntax_tree::Literal;
use nom::{
    branch::alt,
    character::complete::{char, i32, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1},
    number::complete::recognize_float,
    sequence::{preceded, terminated, tuple},
    IResult,
};

/// Represents a type that can be parsed from a string
pub trait Parsable: Sized {
    /// Parses an instance of the calling type from a string. Returns the
    /// unparsed remainder of the input string and the parsed instance.
    fn parse(input: &str) -> IResult<&str, Self>;
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

impl Literal {
    fn parse_int(input: &str) -> IResult<&str, Self> {
        map(i32, |num| Literal::IntegerLiteral(num))(input)
    }

    fn parse_float(input: &str) -> IResult<&str, Self> {
        map_res(
            alt((
                // Case one: .42
                recognize(tuple((
                    char('.'),
                    decimal,
                    opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
                ))), // Case two: 42e42 and 42.42e42
                recognize(tuple((
                    decimal,
                    opt(preceded(char('.'), decimal)),
                    one_of("eE"),
                    opt(one_of("+-")),
                    decimal,
                ))), // Case three: 42. and 42.42
                recognize(tuple((decimal, char('.'), opt(decimal)))),
            )),
            |num| match num.parse() {
                Ok(num) => Ok(Literal::FloatLiteral(num)),
                Err(err) => Err(err),
            },
        )(input)
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
    fn parse_integer_positive() {
        assert_eq!(
            Literal::parse("32"),
            IResult::Ok(("", Literal::IntegerLiteral(32)))
        );
    }

    #[test]
    fn parse_integer_negative() {
        assert_eq!(
            Literal::parse("-32"),
            IResult::Ok(("", Literal::IntegerLiteral(-32)))
        );
    }

}
