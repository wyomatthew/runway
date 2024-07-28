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
    Filter(EvalExpression),
    Alter(Vec<DeclarationExpression>),
    Comp(Function, Vec<(Identifier, Option<AliasExpression>)>, Vec<Identifier>),
    Limit(Literal),
    Sort(Vec<(SortOrder, Identifier)>),
    Dedup(),  // TODO: implement QUINN START HERE
    Top(),    // TODO: implement
    Bin(),    // TODO: implement
    IpLoc(),  // TODO: implement
    Join(),   // TODO: implement
    Tag(),    // TODO: implement
}


#[derive(Debug, PartialEq)]
pub enum EvalExpression {
    Identifier(Identifier),
    Literal(Literal),
    Function(Function),
    Operator(OperatorExpression),
}

/// TODO: implement
#[derive(Debug,PartialEq)]
pub struct DeclarationExpression();

/// TODO: implement
#[derive(Debug,PartialEq)]
pub struct AssignmentExpression(Identifier, Literal);

/// TODO: implement
#[derive(Debug, PartialEq)]
pub struct Function {

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
#[derive(Debug,PartialEq)]
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
