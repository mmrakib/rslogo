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

    Variable(String),
    IntegerLiteral(i32),
}

#[derive(Debug)]
pub enum Command {
    PenUp,
    PenDown,

    Forward(Expression),
    Back(Expression),
    Left(Expression),
    Right(Expression),
    Turn(Expression),

    SetX(i32),
    SetY(i32),
    SetHeading(i32),
    SetPenColor(i32),

    Make(String, Box<Expression>),
    AddAssign(String, Box<Expression>),

    QueryXCor,
    QueryYCor,
    Heading,
    Color,

    IfEq,
    WhileEq,
    Repeat,

    To,
    End,
}
