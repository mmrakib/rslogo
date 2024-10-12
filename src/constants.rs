#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // Turtle control
    PenUp,
    PenDown,
    Forward(i32),
    Back(i32),
    Left(i32),
    Right(i32),
    SetPenColor(u32),
    Turn(i32),
    SetHeading(i32),
    SetX(i32),
    SetY(i32),

    // Variables
    Make(String, String),
    AddAssign(String, String),

    // Queries
    QueryXCor,
    QueryYCor,
    QueryHeading,
    QueryColor,

    // Control structures
    IfEq(String, String, Vec<Command>),
    WhileEq(String, String, Vec<Command>),

    // Arithmetic operators
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),

    // Logical operators
    GreaterThan(String, String),
    LessThan(String, String),
    And(String, String),
    Or(String, String),

    // Procedures
    To(String, Vec<String>, Vec<Command>),
    End,
}
