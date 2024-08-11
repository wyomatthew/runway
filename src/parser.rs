use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1},
    number::complete::recognize_float,
    sequence::{preceded, terminated, tuple},
    IResult,
};

mod literal;

/// Represents a type that can be parsed from a string
pub trait Parsable: Sized {
    /// Parses an instance of the calling type from a string. Returns the
    /// unparsed remainder of the input string and the parsed instance.
    fn parse(input: &str) -> IResult<&str, Self>;
}
