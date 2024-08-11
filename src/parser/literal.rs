use nom::branch::alt;
use nom::IResult;
use nom::character::complete::i32;
use nom::combinator::map;

use crate::syntax_tree::Literal;
use crate::parser::Parsable;


fn parse_int(input: &str) -> IResult<&str, Literal> {
    map(i32, |num| Literal::IntegerLiteral(num))(input)
}

fn parse_float(input: &str) -> IResult<&str, Literal> {
    todo!()
}

fn parse_time(input: &str) -> IResult<&str, Literal> {
    todo!()
}

fn parse_string(input: &str) -> IResult<&str, Literal> {
    todo!()
}

impl Parsable for Literal {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            parse_float,
            parse_int,
            parse_time,
            parse_string,
        ))(input)
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
    }

    #[test]
    fn parse_float_negative() {
        assert_eq!(
            Literal::parse("-32.5"),
            IResult::Ok(("", Literal::FloatLiteral(-32.5)))
        )
    }
}
