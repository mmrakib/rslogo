#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),

    QueryXCor,
    QueryYCor,
    QueryHeading,
    QueryColor,

    Variable(String),
    IntegerLiteral(i32),
}

#[derive(Debug)]
pub enum Command {
    PenUp,
    PenDown,

    Forward(Box<Expression>),
    Back(Box<Expression>),
    Left(Box<Expression>),
    Right(Box<Expression>),
    Turn(Box<Expression>),

    SetX(Box<Expression>),
    SetY(Box<Expression>),
    SetHeading(Box<Expression>),
    SetPenColor(Box<Expression>),

    Make(Identifier, Box<Expression>),
    AddAssign(Identifier, Box<Expression>),

    IfEq(Box<Expression>, Vec<Command>),
    WhileEq(Box<Expression>, Vec<Command>),
    Repeat(Vec<Command>),

    To,
    End,
}
