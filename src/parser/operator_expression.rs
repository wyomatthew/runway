use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::separated_pair, IResult,
};

use crate::syntax_tree::{BinaryOperatorKind, OperatorExpression, UnaryOperatorKind};

use super::Parsable;

impl Parsable for OperatorExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}

impl Parsable for BinaryOperatorKind {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("."), |_| BinaryOperatorKind::Dot),
            map(tag("="), |_| BinaryOperatorKind::Equal),
            map(tag("!="), |_| BinaryOperatorKind::NotEqual),
            map(tag("<"), |_| BinaryOperatorKind::Less),
            map(tag("<="), |_| BinaryOperatorKind::LessEq),
            map(tag(">"), |_| BinaryOperatorKind::Greater),
            map(tag(">="), |_| BinaryOperatorKind::GreaterEq),
            map(tag("and"), |_| BinaryOperatorKind::And),
            map(tag("or"), |_| BinaryOperatorKind::Or),
            map(tag("IN"), |_| BinaryOperatorKind::In),
            map(separated_pair(tag("NOT"), space1, tag("IN")), |_| {
                BinaryOperatorKind::NotIn
            }),
            map(tag("CONTAINS"), |_| BinaryOperatorKind::Contains),
            map(
                separated_pair(tag("NOT"), space1, tag("CONTAINS")),
                |_| BinaryOperatorKind::NotContains,
            ),
            map(tag("~="), |_| BinaryOperatorKind::RegexMatch),
            map(tag("INCIDR6"), |_| BinaryOperatorKind::Incidr6),
            map(separated_pair(tag("NOT"), space1, tag("INCIDR6")), |_| {
                BinaryOperatorKind::NotEqual
            }),
        ))(input)
    }
}

#[cfg(test)]
mod binary_operator_kind_test {
    use super::*;

    #[test]
    fn dot() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Dot)),
            BinaryOperatorKind::parse(".")
        );
    }
}

impl Parsable for UnaryOperatorKind {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}
