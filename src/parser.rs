use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, i32, one_of},
    combinator::{all_consuming, map, map_res, not, opt, recognize},
    multi::{many0, many1},
    number::complete::recognize_float,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::syntax_tree::{
    AliasExpression, AssignmentExpression, BinaryOperator, Config, DeclarationExpression,
    EvalExpression, Function, Identifier, Literal, LocField, Operation, OperatorExpression, Query,
    SortOrder, SourceStatement, SourceType, Stage, TagList, TopQuantifier, UnaryOperator,
};

mod literal;

/// Represents a type that can be parsed from a string
pub trait Parsable: Sized {
    /// Parses an instance of the calling type from a string. Returns the
    /// unparsed remainder of the input string and the parsed instance.
    fn parse(input: &str) -> IResult<&str, Self>;
}

impl Parsable for Query {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for Config {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for SourceStatement {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for SourceType {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for Stage {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for LocField {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for TopQuantifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for EvalExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for DeclarationExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for AssignmentExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for Function {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for AliasExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for Identifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, consumed) = recognize(many1(alt((alphanumeric1, tag("_")))))(input)?;

        not(all_consuming(Literal::parse))(consumed)?;

        Ok((remaining, Identifier(String::from(consumed))))
    }
}

#[cfg(test)]
mod identifier_test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            Ok(("", Identifier(String::from("basicIdentifier")))),
            Identifier::parse("basicIdentifier")
        );
    }

    #[test]
    fn spaces() {
        assert_eq!(
            Ok((" id2", Identifier(String::from("id1")))),
            Identifier::parse("id1 id2")
        );
    }

    #[test]
    fn numbers() {
        assert_eq!(
            Err(nom::Err::Error(nom::error::Error {
                input: "12345678",
                code: nom::error::ErrorKind::Not
            })),
            Identifier::parse("12345678")
        );
        assert_eq!(
            Ok(("", Identifier(String::from("12345678a")))),
            Identifier::parse("12345678a")
        );
        assert_eq!(
            Ok(("", Identifier(String::from("a12345678")))),
            Identifier::parse("a12345678")
        );
    }

    #[test]
    fn special_chars() {
        assert_eq!(
            Ok((".column", Identifier(String::from("table")))),
            Identifier::parse("table.column")
        );
        assert_eq!(
            Ok(("-column", Identifier(String::from("table")))),
            Identifier::parse("table-column")
        );
        assert_eq!(
            Ok(("", Identifier(String::from("table_column")))),
            Identifier::parse("table_column")
        );
        assert_eq!(
            Ok(("$column", Identifier(String::from("table")))),
            Identifier::parse("table$column")
        );
    }
}

impl Parsable for SortOrder {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for OperatorExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for BinaryOperator {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for UnaryOperator {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for TagList {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}
