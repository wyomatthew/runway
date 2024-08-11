use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{all_consuming, not, recognize},
    multi::many1,
    IResult,
};

use crate::syntax_tree::{Identifier, Literal};

use super::Parsable;

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
