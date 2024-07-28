#[derive(Debug, PartialEq)]
pub struct Query(Option<Config>, SourceStatement, Vec<Operation>);

/// TODO: implement QUINN
#[derive(Debug, PartialEq)]
pub struct Config();

/// TODO: implement MATTHEW
#[derive(Debug, PartialEq)]
pub struct SourceStatement {}

#[derive(Debug, PartialEq)]
enum SourceType {
    Datamodel,
    DatamodelDataset,
    Dataset,
    Preset,
    ColdDataset
}

#[derive(Debug, PartialEq)]
pub struct Operation(Stage);

#[derive(Debug, PartialEq)]
pub enum Stage {
    Fields(Vec<(Identifier, Option<AliasExpression>)>),
    Filter(EvalExpression),
    Alter(Vec<DeclarationExpression>),
    Comp(
        Function,
        Vec<(Identifier, Option<AliasExpression>)>,
        Vec<Identifier>,
    ),
    Limit(Literal),
    Sort(Vec<(SortOrder, Identifier)>),
    Dedup((Vec<Identifier>, Option<(SortOrder, Identifier)>)),
    Top(
        (
            Option<Literal>,
            Identifier,
            Option<(
                Vec<Identifier>,
                Option<Vec<(TopQuantifier, AliasExpression)>>,
            )>,
        ),
    ),
    Bin((Identifier, Vec<AssignmentExpression>)),
    IpLoc((Identifier, Option<Vec<(LocField, AliasExpression)>>)),
    Join(
        (
            Vec<AssignmentExpression>,
            Query,
            AliasExpression,
            EvalExpression,
        ),
    ),
    Tag(TagList),
}

#[derive(Debug, PartialEq)]
pub enum LocField {
    City,
    Continent,
    Country,
    LatLon,
    Region,
    Timezone,
}

#[derive(Debug, PartialEq)]
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

/// TODO: implement MATTHEW
#[derive(Debug, PartialEq)]
pub struct DeclarationExpression();

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression(Identifier, Literal);

/// TODO: implement MATTHEW
#[derive(Debug, PartialEq)]
pub struct Function {}

#[derive(Debug, PartialEq)]
pub struct AliasExpression {}

/// TODO: implement QUINN
#[derive(Debug, PartialEq)]
pub struct Identifier {}

/// TODO: implement MATTHEW
#[derive(Debug, PartialEq)]
pub enum Literal {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    TimeLiteral(), // TODO: implement QUINN
}

#[derive(Debug, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
pub enum OperatorExpression {
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
}

/// TODO: implement MATTHEW
#[derive(Debug, PartialEq)]
pub struct BinaryOperator {}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug, PartialEq)]
pub struct TagList(Vec<Literal>);

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
