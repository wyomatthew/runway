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
    Dedup(),  // TODO: implement QUINN START HERE
    Top(),    // TODO: implement
    Bin(),    // TODO: implement
    IpLoc(),  // TODO: implement
    Join(),   // TODO: implement
    Tag(),    // TODO: implement
}

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
