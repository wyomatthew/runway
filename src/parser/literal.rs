use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::IResult;

use crate::parser::Parsable;
use crate::syntax_tree::Literal;

fn parse_int(input: &str) -> IResult<&str, Literal> {
    map(i32, |num| Literal::IntegerLiteral(num))(input)
}

fn decimal(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of("0123456789"))(input)
}

fn parse_float(input: &str) -> IResult<&str, Literal> {
    map_res(
        recognize(tuple((opt(tag("-")), opt(decimal), tag("."), decimal))),
        |num| num.parse().map(Literal::FloatLiteral),
    )(input)
}

fn parse_time(input: &str) -> IResult<&str, Literal> {
    todo!()
}

fn parse_string(input: &str) -> IResult<&str, Literal> {
    todo!()
}

impl Parsable for Literal {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((parse_float, parse_int, parse_time, parse_string))(input)
    }
}

#[cfg(test)]
mod test {
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

    #[test]
    fn parse_float_positive() {
        assert_eq!(
            Literal::parse("32.5"),
            IResult::Ok(("", Literal::FloatLiteral(32.5)))
        );
        assert_eq!(
            Literal::parse(".5"),
            IResult::Ok(("", Literal::FloatLiteral(0.5)))
        );
    }

    #[test]
    fn parse_float_negative() {
        assert_eq!(
            Literal::parse("-32.5"),
            IResult::Ok(("", Literal::FloatLiteral(-32.5)))
        );
        assert_eq!(
            Literal::parse("-.5"),
            IResult::Ok(("", Literal::FloatLiteral(-0.5)))
        );
    }
}
