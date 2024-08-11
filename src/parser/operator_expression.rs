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
            map(tag("."), |text| BinaryOperatorKind::Dot),
            map(tag("="), |text| BinaryOperatorKind::Equal),
            map(tag("!="), |text| BinaryOperatorKind::NotEqual),
            map(tag("<"), |text| BinaryOperatorKind::Less),
            map(tag("<="), |text| BinaryOperatorKind::LessEq),
            map(tag(">"), |text| BinaryOperatorKind::Greater),
            map(tag(">="), |text| BinaryOperatorKind::GreaterEq),
            map(tag("and"), |text| BinaryOperatorKind::And),
            map(tag("or"), |text| BinaryOperatorKind::Or),
            map(tag("IN"), |text| BinaryOperatorKind::In),
            map(separated_pair(tag("NOT"), space1, tag("IN")), |text| {
                BinaryOperatorKind::NotIn
            }),
            map(tag("CONTAINS"), |text| BinaryOperatorKind::Contains),
            map(
                separated_pair(tag("NOT"), space1, tag("CONTAINS")),
                |text| BinaryOperatorKind::NotContains,
            ),
            map(tag("~="), |text| BinaryOperatorKind::RegexMatch),
            map(tag("INCIDR6"), |text| BinaryOperatorKind::Incidr6),
            map(separated_pair(tag("NOT"), space1, tag("INCIDR6")), |text| {
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
