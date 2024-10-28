/**
 * A block of statements in the Logo language
 * 
 * Represents a sequence of statements that are to be executed in order
 * Every Logo program is viewed as a nested array of statement blocks, executed in order with inner blocks taking precedence
 */
pub type Block = Vec<Statement>;

/**
 * An identifier in the Logo language
 * 
 * Represents a name for referring to another code feature i.e. variables, procedures
 * 
 * The argument structure is as follows:
 * name: String - The name of the identifier
 * access_modifier: String - The access modifier (i.e. '"', ':' or ''), used to determine evaluation logic
 */
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String, pub String);

/**
 * An expression in the Logo language
 * 
 * Represents all types of operations/values that can be evaluated
 * 
 * The argument structure of a binary operation is as follows:
 * lhs: Box<Expression> - The left hand side of the operation
 * rhs: Box<Expression> - The right hand side of the operation
 * 
 * Terminals values have one argument, representing a Rust-intepretable version of the value itself
 * e.g. IntegerLiteral(5) is defined as 'IntegerLiteral' for the parser and 5 for the evaluator
 * 
 * Queries have no arguments as they evaluate to terminal values anyways
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /*
     * Arithmetic operations
     */
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),

    /*
     * Logical operations
     */
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    /*
     * Comparison operations
     */
    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),

    /*
     * Queries
     */
    QueryXCor,
    QueryYCor,
    QueryHeading,
    QueryColor,

    /*
     * Terminal values
     */
    VariableReference(String),
    StringLiteral(String),
    IntegerLiteral(i32),
}

/**
 * A statement in the Logo language
 * 
 * Represents a single command to be executed
 * Usually takes a whole line, and is preceded by a keyword
 * e.g. PENDOWN, FORWARD, IF, SETHEADING, etc.
 * 
 * Takes a variety of arguments, depending on the type of statement
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /*
     * Pen control
     */
    PenUp,
    PenDown,

    /*
     * Movement control
     */
    Forward(Box<Expression>),
    Back(Box<Expression>),
    Left(Box<Expression>),
    Right(Box<Expression>),
    Turn(Box<Expression>),

    /*
     * Setters
     */
    SetX(Box<Expression>),
    SetY(Box<Expression>),
    SetHeading(Box<Expression>),
    SetPenColor(Box<Expression>),

    /*
     * Variable assignment
     */
    Make(Identifier, Box<Expression>),
    AddAssign(Identifier, Box<Expression>),

    /*
     * Control structures
     */
    If(Box<Expression>, Box<Block>),
    While(Box<Expression>, Box<Block>),
    Repeat(Box<Expression>, Box<Block>),

    /*
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
