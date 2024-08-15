use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, space1},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::syntax_tree::{
    BinaryOperatorKind, EvalExpression, OperatorExpression, UnaryOperatorKind,
};

use super::Parsable;

fn parse_binary_operator_expression(input: &str) -> IResult<&str, OperatorExpression> {
    let (unparsed, (expression_left, _, operator_kind, _, expression_right)) = tuple((
        EvalExpression::parse,
        multispace1,
        BinaryOperatorKind::parse,
        multispace1,
        EvalExpression::parse,
    ))(input)?;

    Ok((
        unparsed,
        OperatorExpression::BinaryOperator(
            Box::new(expression_left),
            Box::new(expression_right),
            operator_kind,
        ),
    ))
}

fn parse_unary_operator_expression(input: &str) -> IResult<&str, OperatorExpression> {
    let (unparsed, (operator_kind, expression)) =
        separated_pair(UnaryOperatorKind::parse, multispace1, EvalExpression::parse)(input)?;

    Ok((
        unparsed,
        OperatorExpression::UnaryOperator(Box::new(expression), operator_kind),
    ))
}

impl Parsable for OperatorExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            parse_binary_operator_expression,
            parse_unary_operator_expression,
        ))(input)
    }
}

impl Parsable for BinaryOperatorKind {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("."), |_| BinaryOperatorKind::Dot),
            map(tag("="), |_| BinaryOperatorKind::Equal),
            map(tag("!="), |_| BinaryOperatorKind::NotEqual),
            map(tag("<="), |_| BinaryOperatorKind::LessEq),
            map(tag("<"), |_| BinaryOperatorKind::Less),
            map(tag(">="), |_| BinaryOperatorKind::GreaterEq),
            map(tag(">"), |_| BinaryOperatorKind::Greater),
            map(tag("and"), |_| BinaryOperatorKind::And),
            map(tag("or"), |_| BinaryOperatorKind::Or),
            map(tag("INCIDR6"), |_| BinaryOperatorKind::Incidr6),
            map(separated_pair(tag("NOT"), space1, tag("INCIDR6")), |_| {
                BinaryOperatorKind::NotIncidr6
            }),
            map(tag("IN"), |_| BinaryOperatorKind::In),
            map(separated_pair(tag("NOT"), space1, tag("IN")), |_| {
                BinaryOperatorKind::NotIn
            }),
            map(tag("CONTAINS"), |_| BinaryOperatorKind::Contains),
            map(separated_pair(tag("NOT"), space1, tag("CONTAINS")), |_| {
                BinaryOperatorKind::NotContains
            }),
            map(tag("~="), |_| BinaryOperatorKind::RegexMatch),
        ))(input)
    }
}

#[cfg(test)]
mod binary_operator_kind_test {
    use super::*;

    #[test]
    fn test_dot() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Dot)),
            BinaryOperatorKind::parse(".")
        );
    }

    #[test]
    fn test_equal() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Equal)),
            BinaryOperatorKind::parse("=")
        );
    }

    #[test]
    fn test_not_equal() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::NotEqual)),
            BinaryOperatorKind::parse("!=")
        );
    }

    #[test]
    fn test_less() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Less)),
            BinaryOperatorKind::parse("<")
        );
    }

    #[test]
    fn test_less_eq() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::LessEq)),
            BinaryOperatorKind::parse("<=")
        );
    }

    #[test]
    fn test_greater() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Greater)),
            BinaryOperatorKind::parse(">")
        );
    }

    #[test]
    fn test_greater_eq() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::GreaterEq)),
            BinaryOperatorKind::parse(">=")
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::And)),
            BinaryOperatorKind::parse("and")
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Or)),
            BinaryOperatorKind::parse("or")
        );
    }

    #[test]
    fn test_in() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::In)),
            BinaryOperatorKind::parse("IN")
        );
    }

    #[test]
    fn test_not_in() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::NotIn)),
            BinaryOperatorKind::parse("NOT IN")
        );
    }

    #[test]
    fn test_contains() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Contains)),
            BinaryOperatorKind::parse("CONTAINS")
        );
    }

    #[test]
    fn test_not_contains() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::NotContains)),
            BinaryOperatorKind::parse("NOT CONTAINS")
        );
    }

    #[test]
    fn test_regex_match() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::RegexMatch)),
            BinaryOperatorKind::parse("~=")
        );
    }

    #[test]
    fn test_incidr6() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::Incidr6)),
            BinaryOperatorKind::parse("INCIDR6")
        );
    }

    #[test]
    fn test_not_incidr6() {
        assert_eq!(
            Ok(("", BinaryOperatorKind::NotIncidr6)),
            BinaryOperatorKind::parse("NOT INCIDR6")
        );
    }
}

impl Parsable for UnaryOperatorKind {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }
}
