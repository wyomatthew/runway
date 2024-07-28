#[derive(Debug, PartialEq)]
pub struct Query(Option<Config>, SourceStatement, Vec<Operation>);

/// TODO: implement
#[derive(Debug, PartialEq)]
pub struct Config {}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub struct SourceStatement {}

/// TODO: implement
#[derive(Debug, PartialEq)]
enum SourceType {}

#[derive(Debug, PartialEq)]
pub struct Operation(Stage);

#[derive(Debug, PartialEq)]
pub enum Stage {
    Fields(Vec<(Identifier, Option<AliasExpression>)>),
    Filter(), // TODO: implement
    Alter(),  // TODO: implement
    Comp(),   // TODO: implement
    Limit(),  // TODO: implement
    Sort(),   // TODO: implement
    Dedup((Vec<Indentifier>, Option<(SortOrder, Identifier)>)),
    Top(
        (
            Option<Literal>,
            Identifier,
            Option<Vec<Identifier>, Option<Vec<(TopQuantifier, AliasExpression)>>>,
        ),
    ),
    Bin((Identifier, Vec<AssignmentExpression>)), 
    IpLoc((Identifier, Optional<Vect<(LocField, AliasExpression)>>)), 
    Join(),  // TODO: implement
    Tag(),   // TODO: implement
}

#[derive(Debug,PartialEq)]
pub enum LocField {
    City,
    Continent,
    Country,
    LatLon,
    Region,
    Timezone
}

#[derive(Debug,PartialEq)]
pub enum TopQuantifier {
    TopCount,
    TopPercent,
}

#[derive(Debug, PartialEq)]
pub enum EvalExpression {
    Identifier(Identifier),
    Literal(Literal),
    Function(Function),
    Operator(OperatorExpression),
}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub struct Function {}

#[derive(Debug, PartialEq)]
pub struct AliasExpression {}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub struct Identifier {}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub enum Literal {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    TimeLiteral(), // TODO: implement
}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub enum OperatorExpression {
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub struct BinaryOperator {}

/// TODO: implement
#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
}

impl UnaryOperator {
    fn get_value(&self) -> &'static str {
        match self {
            UnaryOperator::Not => "Not",
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 4;
        assert_eq!(result, 6);
    }
}
