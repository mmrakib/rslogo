pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String, pub String);

#[derive(Debug, PartialEq)]
pub enum Expression {
    /**
     * Arithmetic operators
     */
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),

    /**
     * Logical operators
     */
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    /**
     * Comparison operators
     */
    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),

    /**
     * Queries
     */
    QueryXCor,
    QueryYCor,
    QueryHeading,
    QueryColor,

    /**
     * Terminal values
     */
    VariableReference(String),
    StringLiteral(String),
    IntegerLiteral(i32),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    /**
     * Pen control
     */
    PenUp,
    PenDown,

    /**
     * Turtle movement control
     */
    Forward(Box<Expression>),
    Back(Box<Expression>),
    Left(Box<Expression>),
    Right(Box<Expression>),
    Turn(Box<Expression>),

    /**
     * Setters
     */
    SetX(Box<Expression>),
    SetY(Box<Expression>),
    SetHeading(Box<Expression>),
    SetPenColor(Box<Expression>),

    /**
     * Variable assignment
     */
    Make(Identifier, Box<Expression>),
    AddAssign(Identifier, Box<Expression>),

    /**
     * Control structures
     */
    If(Box<Expression>, Box<Block>),
    While(Box<Expression>, Box<Block>),
    Repeat(Box<Expression>, Box<Block>),

    /**
     * Procedures
     */
    ProcedureDefinition {
        name: Identifier,
        parameters: Vec<Expression>,
        body: Block,
    },
    ProcedureCall {
        name: Identifier,
        arguments: Vec<Expression>,
    }
}
