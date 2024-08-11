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

mod identifier;

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
