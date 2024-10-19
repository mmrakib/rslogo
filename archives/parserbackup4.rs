use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Forward(Expression),
    Back(Expression),
    Left(Expression),
    Right(Expression),
    PenUp,
    PenDown,
    HideTurtle,
    ShowTurtle,
    SetX(Expression),
    SetY(Expression),
    SetXY(Expression, Expression),
    Home,
    ClearScreen,
    Make(String, Expression),
    Procedure(String, Vec<String>, Vec<Statement>),
    Call(String, Vec<Expression>),
    Repeat(Expression, Vec<Statement>),
    If(Expression, Vec<Statement>),
    IfElse(Expression, Vec<Statement>, Vec<Statement>),
    While(Expression, Vec<Statement>),
    For(String, Expression, Expression, Expression, Vec<Statement>),
    Comment(String),
    // Add more commands and structures as needed
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Variable(String),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    UnaryOp(Operator, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    // Add more operators as needed
}

/// Parse a number (integer or floating point)
fn parse_number(input: &str) -> IResult<&str, Expression> {
    map_res(
        recognize(tuple((
            opt(one_of("+-")),
            digit1,
            opt(tuple((tag("."), digit1))),
        ))),
        |s: &str| s.parse::<f64>().map(Expression::Number),
    )(input)
}

/// Parse an identifier (variable or procedure name)
fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

/// Parse an expression (supports variables, numbers, and binary operations)
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_logical_or(input)
}

/// Parse logical OR expressions
fn parse_logical_or(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_logical_and(input)?;
    fold_many0(
        pair(
            preceded(multispace0, alt((tag("or"), tag("OR")))),
            preceded(multispace0, parse_logical_and),
        ),
        move || init.clone(),
        |acc, (_, expr)| {
            Expression::BinaryOp(Box::new(acc), Operator::Or, Box::new(expr))
        },
    )(input)
}

/// Parse logical AND expressions
fn parse_logical_and(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_equality(input)?;
    fold_many0(
        pair(
            preceded(multispace0, alt((tag("and"), tag("AND")))),
            preceded(multispace0, parse_equality),
        ),
        move || init.clone(),
        |acc, (_, expr)| {
            Expression::BinaryOp(Box::new(acc), Operator::And, Box::new(expr))
        },
    )(input)
}

/// Parse equality expressions
fn parse_equality(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_comparison(input)?;
    fold_many0(
        pair(
            preceded(multispace0, alt((tag("="), tag("<>")))),
            preceded(multispace0, parse_comparison),
        ),
        move || init.clone(),
        |acc, (op, expr)| {
            let operator = if op == "=" {
                Operator::Equal
            } else {
                Operator::NotEqual
            };
            Expression::BinaryOp(Box::new(acc), operator, Box::new(expr))
        },
    )(input)
}

/// Parse comparison expressions
fn parse_comparison(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_term(input)?;
    fold_many0(
        pair(
            preceded(multispace0, alt((tag("<"), tag(">")))),
            preceded(multispace0, parse_term),
        ),
        move || init.clone(),
        |acc, (op, expr)| {
            let operator = if op == "<" {
                Operator::LessThan
            } else {
                Operator::GreaterThan
            };
            Expression::BinaryOp(Box::new(acc), operator, Box::new(expr))
        },
    )(input)
}

/// Parse addition and subtraction
fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_factor(input)?;
    fold_many0(
        pair(
            preceded(multispace0, one_of("+-")),
            preceded(multispace0, parse_factor),
        ),
        move || init.clone(),
        |acc, (op, expr)| {
            let operator = if op == '+' {
                Operator::Add
            } else {
                Operator::Subtract
            };
            Expression::BinaryOp(Box::new(acc), operator, Box::new(expr))
        },
    )(input)
}

/// Parse multiplication and division
fn parse_factor(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_unary(input)?;
    fold_many0(
        pair(
            preceded(multispace0, one_of("*/")),
            preceded(multispace0, parse_unary),
        ),
        move || init.clone(),
        |acc, (op, expr)| {
            let operator = if op == '*' {
                Operator::Multiply
            } else {
                Operator::Divide
            };
            Expression::BinaryOp(Box::new(acc), operator, Box::new(expr))
        },
    )(input)
}

/// Parse unary operators
fn parse_unary(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            pair(
                preceded(
                    multispace0,
                    alt((tag("-"), tag("not"), tag("NOT"))),
                ),
                parse_unary,
            ),
            |(op, expr)| {
                let operator = match op {
                    "-" => Operator::Subtract,
                    _ => Operator::Not,
                };
                Expression::UnaryOp(operator, Box::new(expr))
            },
        ),
        parse_primary,
    ))(input)
}

/// Parse primary expressions
fn parse_primary(input: &str) -> IResult<&str, Expression> {
    alt((
        delimited(
            preceded(multispace0, tag("(")),
            parse_expression,
            preceded(multispace0, tag(")")),
        ),
        parse_function_call,
        parse_variable,
        parse_number,
    ))(input)
}

/// Parse a variable
fn parse_variable(input: &str) -> IResult<&str, Expression> {
    map(preceded(tag(":"), parse_identifier), Expression::Variable)(input)
}

/// Parse a function call
fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    let (input, name) = parse_identifier(input)?;
    let (input, args) = delimited(
        preceded(multispace0, tag("(")),
        separated_list0(
            preceded(multispace0, tag(",")),
            preceded(multispace0, parse_expression),
        ),
        preceded(multispace0, tag(")")),
    )(input)?;
    Ok((
        input,
        Expression::FunctionCall(name, args),
    ))
}

/// Parse the 'make' command (variable assignment)
fn parse_make(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("make"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, var_name) = parse_literal_string(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expression(input)?;
    Ok((input, Statement::Make(var_name, expr)))
}

/// Parse a literal string (used for variable names in 'make')
fn parse_literal_string(input: &str) -> IResult<&str, String> {
    alt((
        delimited(tag("\""), parse_identifier, multispace0),
        parse_identifier,
    ))(input)
}

/// Parse a comment
fn parse_comment(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag(";"))(input)?;
    let (input, comment) = take_until("\n")(input)?;
    Ok((input, Statement::Comment(comment.trim().to_string())))
}

/// Parse procedure definition
fn parse_procedure(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("to"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, params) = many0(preceded(multispace1, parse_variable_name))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = many0(alt((
        parse_statement,
        parse_comment,
    )))(input)?;
    let (input, _) = preceded(multispace0, tag_no_case("end"))(input)?;
    Ok((input, Statement::Procedure(name, params, body)))
}

/// Parse a variable name (without the ':' prefix)
fn parse_variable_name(input: &str) -> IResult<&str, String> {
    map(parse_literal_string, |s| s)(input)
}

/// Parse procedure call
fn parse_procedure_call(input: &str) -> IResult<&str, Statement> {
    let (input, name) = parse_identifier(input)?;
    let (input, args) = many0(preceded(multispace1, parse_expression))(input)?;
    Ok((input, Statement::Call(name, args)))
}

/// Parse control structures (if, ifelse, while, for)
fn parse_if(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("if"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, statements) = delimited(
        preceded(multispace0, tag("[")),
        many0(parse_statement),
        preceded(multispace0, tag("]")),
    )(input)?;
    Ok((input, Statement::If(condition, statements)))
}

fn parse_ifelse(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("ifelse"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, true_statements) = delimited(
        preceded(multispace0, tag("[")),
        many0(parse_statement),
        preceded(multispace0, tag("]")),
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, false_statements) = delimited(
        preceded(multispace0, tag("[")),
        many0(parse_statement),
        preceded(multispace0, tag("]")),
    )(input)?;
    Ok((
        input,
        Statement::IfElse(condition, true_statements, false_statements),
    ))
}

fn parse_while(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("while"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, statements) = delimited(
        preceded(multispace0, tag("[")),
        many0(parse_statement),
        preceded(multispace0, tag("]")),
    )(input)?;
    Ok((input, Statement::While(condition, statements)))
}

fn parse_for(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("for"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, var_name) = parse_literal_string(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("from")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, start) = parse_expression(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("to")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, end) = parse_expression(input)?;
    let (input, step) = opt(preceded(
        tuple((multispace1, tag_no_case("by"), multispace1)),
        parse_expression,
    ))(input)?;
    let step_expr = step.unwrap_or(Expression::Number(1.0));
    let (input, _) = multispace0(input)?;
    let (input, statements) = delimited(
        preceded(multispace0, tag("[")),
        many0(parse_statement),
        preceded(multispace0, tag("]")),
    )(input)?;
    Ok((
        input,
        Statement::For(var_name, start, end, step_expr, statements),
    ))
}

/// Parse any single statement
fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt_statements(input)
}

fn alt_statements(input: &str) -> IResult<&str, Statement> {
    // Split parsers into groups to avoid the 21-item limit
    let group1 = alt((
        parse_make,
        parse_forward,
        parse_back,
        parse_left,
        parse_right,
        parse_penup,
        parse_pendown,
        parse_hide_turtle,
        parse_show_turtle,
        parse_setx,
        parse_sety,
        parse_setxy,
        parse_home,
        parse_clearscreen,
        parse_repeat,
    ));

    let group2 = alt((
        parse_ifelse,
        parse_if,
        parse_while,
        parse_for,
        parse_procedure,
        parse_comment,
        parse_procedure_call,
    ));

    alt((group1, group2))(input)
}

/// Parse the 'forward' command
fn parse_forward(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(
        multispace0,
        alt((tag_no_case("forward"), tag_no_case("fd"))),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = parse_expression(input)?;
    Ok((input, Statement::Forward(distance)))
}

/// Parse the 'back' command
fn parse_back(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(
        multispace0,
        alt((tag_no_case("back"), tag_no_case("bk"))),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = parse_expression(input)?;
    Ok((input, Statement::Back(distance)))
}

/// Parse the 'left' command
fn parse_left(input: &str) -> IResult<&str, Statement> {
    let (input, _) =
        preceded(multispace0, alt((tag_no_case("left"), tag_no_case("lt"))))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, angle) = parse_expression(input)?;
    Ok((input, Statement::Left(angle)))
}

/// Parse the 'right' command
fn parse_right(input: &str) -> IResult<&str, Statement> {
    let (input, _) =
        preceded(multispace0, alt((tag_no_case("right"), tag_no_case("rt"))))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, angle) = parse_expression(input)?;
    Ok((input, Statement::Right(angle)))
}

/// Parse the 'penup' command
fn parse_penup(input: &str) -> IResult<&str, Statement> {
    let (input, _) =
        preceded(multispace0, alt((tag_no_case("penup"), tag_no_case("pu"))))(input)?;
    Ok((input, Statement::PenUp))
}

/// Parse the 'pendown' command
fn parse_pendown(input: &str) -> IResult<&str, Statement> {
    let (input, _) =
        preceded(multispace0, alt((tag_no_case("pendown"), tag_no_case("pd"))))(input)?;
    Ok((input, Statement::PenDown))
}

/// Parse the 'hideturtle' command
fn parse_hide_turtle(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(
        multispace0,
        alt((tag_no_case("hideturtle"), tag_no_case("ht"))),
    )(input)?;
    Ok((input, Statement::HideTurtle))
}

/// Parse the 'showturtle' command
fn parse_show_turtle(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(
        multispace0,
        alt((tag_no_case("showturtle"), tag_no_case("st"))),
    )(input)?;
    Ok((input, Statement::ShowTurtle))
}

/// Parse the 'setx' command
fn parse_setx(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("setx"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, x) = parse_expression(input)?;
    Ok((input, Statement::SetX(x)))
}

/// Parse the 'sety' command
fn parse_sety(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("sety"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, y) = parse_expression(input)?;
    Ok((input, Statement::SetY(y)))
}

/// Parse the 'setxy' command
fn parse_setxy(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("setxy"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, x) = parse_expression(input)?;
    let (input, _) = multispace1(input)?;
    let (input, y) = parse_expression(input)?;
    Ok((input, Statement::SetXY(x, y)))
}

/// Parse the 'home' command
fn parse_home(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("home"))(input)?;
    Ok((input, Statement::Home))
}

/// Parse the 'clearscreen' command
fn parse_clearscreen(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(
        multispace0,
        alt((tag_no_case("clearscreen"), tag_no_case("cs"))),
    )(input)?;
    Ok((input, Statement::ClearScreen))
}

/// Parse the 'repeat' control structure
fn parse_repeat(input: &str) -> IResult<&str, Statement> {
    let (input, _) = preceded(multispace0, tag_no_case("repeat"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, count) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, statements) = delimited(
        preceded(multispace0, tag("[")),
        many0(parse_statement),
        preceded(multispace0, tag("]")),
    )(input)?;
    Ok((input, Statement::Repeat(count, statements)))
}

/// Parse an entire program (sequence of statements)
fn parse_program(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, _) = multispace0(input)?;
    let (input, statements) = many0(alt((
        parse_statement,
        parse_comment,
    )))(input)?;
    Ok((input, statements))
}

// Include your unit tests and main function as needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(
            parse_number("123"),
            Ok(("", Expression::Number(123.0)))
        );
        assert_eq!(
            parse_number("-45.67"),
            Ok(("", Expression::Number(-45.67)))
        );
        assert_eq!(
            parse_number("+89"),
            Ok(("", Expression::Number(89.0)))
        );
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            parse_identifier("variable_name"),
            Ok(("", "variable_name".to_string()))
        );
        assert_eq!(
            parse_identifier("_hidden"),
            Ok(("", "_hidden".to_string()))
        );
        assert_eq!(
            parse_identifier("var123"),
            Ok(("", "var123".to_string()))
        );
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(
            parse_variable(":x"),
            Ok(("", Expression::Variable("x".to_string())))
        );
        assert_eq!(
            parse_variable(":variable_name"),
            Ok(("", Expression::Variable("variable_name".to_string())))
        );
    }

    #[test]
    fn test_parse_expression() {
        // Simple number
        assert_eq!(
            parse_expression("42"),
            Ok(("", Expression::Number(42.0)))
        );

        // Variable
        assert_eq!(
            parse_expression(":x"),
            Ok(("", Expression::Variable("x".to_string())))
        );

        // Addition
        assert_eq!(
            parse_expression("2 + 3"),
            Ok((
                "",
                Expression::BinaryOp(
                    Box::new(Expression::Number(2.0)),
                    Operator::Add,
                    Box::new(Expression::Number(3.0))
                )
            ))
        );

        // Complex expression
        assert_eq!(
            parse_expression("2 + 3 * :x - (4 / :y)"),
            Ok((
                "",
                Expression::BinaryOp(
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(2.0)),
                        Operator::Add,
                        Box::new(Expression::BinaryOp(
                            Box::new(Expression::Number(3.0)),
                            Operator::Multiply,
                            Box::new(Expression::Variable("x".to_string()))
                        ))
                    )),
                    Operator::Subtract,
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(4.0)),
                        Operator::Divide,
                        Box::new(Expression::Variable("y".to_string()))
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_parse_make() {
        let input = "make \"length 100";
        assert_eq!(
            parse_make(input),
            Ok((
                "",
                Statement::Make("length".to_string(), Expression::Number(100.0))
            ))
        );

        let input = "make \"size :x + 50";
        assert_eq!(
            parse_make(input),
            Ok((
                "",
                Statement::Make(
                    "size".to_string(),
                    Expression::BinaryOp(
                        Box::new(Expression::Variable("x".to_string())),
                        Operator::Add,
                        Box::new(Expression::Number(50.0))
                    )
                )
            ))
        );
    }

    #[test]
    fn test_parse_forward() {
        let input = "forward 100";
        assert_eq!(
            parse_forward(input),
            Ok((
                "",
                Statement::Forward(Expression::Number(100.0))
            ))
        );

        let input = "fd :distance";
        assert_eq!(
            parse_forward(input),
            Ok((
                "",
                Statement::Forward(Expression::Variable("distance".to_string()))
            ))
        );
    }

    #[test]
    fn test_parse_repeat() {
        let input = "repeat 4 [ forward 100 right 90 ]";
        assert_eq!(
            parse_repeat(input),
            Ok((
                "",
                Statement::Repeat(
                    Expression::Number(4.0),
                    vec![
                        Statement::Forward(Expression::Number(100.0)),
                        Statement::Right(Expression::Number(90.0))
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_parse_procedure() {
        let input = r#"
            to square :size
                repeat 4 [
                    forward :size
                    right 90
                ]
            end
        "#;

        assert_eq!(
            parse_procedure(input),
            Ok((
                "",
                Statement::Procedure(
                    "square".to_string(),
                    vec!["size".to_string()],
                    vec![Statement::Repeat(
                        Expression::Number(4.0),
                        vec![
                            Statement::Forward(Expression::Variable("size".to_string())),
                            Statement::Right(Expression::Number(90.0)),
                        ]
                    )]
                )
            ))
        );
    }

    #[test]
    fn test_parse_if() {
        let input = r#"
            if :x > 10 [
                forward 100
            ]
        "#;

        assert_eq!(
            parse_if(input),
            Ok((
                "",
                Statement::If(
                    Expression::BinaryOp(
                        Box::new(Expression::Variable("x".to_string())),
                        Operator::GreaterThan,
                        Box::new(Expression::Number(10.0))
                    ),
                    vec![Statement::Forward(Expression::Number(100.0))]
                )
            ))
        );
    }

    #[test]
    fn test_parse_ifelse() {
        let input = r#"
            ifelse :x = 0 [
                print "Zero
            ] [
                print "Non-zero
            ]
        "#;

        assert_eq!(
            parse_ifelse(input),
            Ok((
                "",
                Statement::IfElse(
                    Expression::BinaryOp(
                        Box::new(Expression::Variable("x".to_string())),
                        Operator::Equal,
                        Box::new(Expression::Number(0.0))
                    ),
                    vec![Statement::Call(
                        "print".to_string(),
                        vec![Expression::Variable("Zero".to_string())]
                    )],
                    vec![Statement::Call(
                        "print".to_string(),
                        vec![Expression::Variable("Non-zero".to_string())]
                    )]
                )
            ))
        );
    }

    #[test]
    fn test_parse_comment() {
        let input = "; This is a comment\nforward 100";
        assert_eq!(
            parse_comment(input),
            Ok((
                "\nforward 100",
                Statement::Comment(" This is a comment".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_program() {
        let input = r#"
            ; Draw a square
            to square :size
                repeat 4 [
                    forward :size
                    right 90
                ]
            end

            make "length 100
            square :length

            if :length > 50 [
                print "Large square
            ] else [
                print "Small square
            ]
        "#;

        let expected = vec![
            Statement::Comment(" Draw a square".to_string()),
            Statement::Procedure(
                "square".to_string(),
                vec!["size".to_string()],
                vec![Statement::Repeat(
                    Expression::Number(4.0),
                    vec![
                        Statement::Forward(Expression::Variable("size".to_string())),
                        Statement::Right(Expression::Number(90.0)),
                    ],
                )],
            ),
            Statement::Make(
                "length".to_string(),
                Expression::Number(100.0),
            ),
            Statement::Call(
                "square".to_string(),
                vec![Expression::Variable("length".to_string())],
            ),
            Statement::IfElse(
                Expression::BinaryOp(
                    Box::new(Expression::Variable("length".to_string())),
                    Operator::GreaterThan,
                    Box::new(Expression::Number(50.0)),
                ),
                vec![Statement::Call(
                    "print".to_string(),
                    vec![Expression::Variable("Large square".to_string())],
                )],
                vec![Statement::Call(
                    "print".to_string(),
                    vec![Expression::Variable("Small square".to_string())],
                )],
            ),
        ];

        assert_eq!(parse_program(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_function_call() {
        let input = "sin(90)";
        assert_eq!(
            parse_function_call(input),
            Ok((
                "",
                Expression::FunctionCall(
                    "sin".to_string(),
                    vec![Expression::Number(90.0)]
                )
            ))
        );

        let input = "max(:a, :b)";
        assert_eq!(
            parse_function_call(input),
            Ok((
                "",
                Expression::FunctionCall(
                    "max".to_string(),
                    vec![
                        Expression::Variable("a".to_string()),
                        Expression::Variable("b".to_string())
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_parse_procedure_call() {
        let input = "square 100";
        assert_eq!(
            parse_procedure_call(input),
            Ok((
                "",
                Statement::Call(
                    "square".to_string(),
                    vec![Expression::Number(100.0)]
                )
            ))
        );

        let input = "drawShape :sides :length";
        assert_eq!(
            parse_procedure_call(input),
            Ok((
                "",
                Statement::Call(
                    "drawShape".to_string(),
                    vec![
                        Expression::Variable("sides".to_string()),
                        Expression::Variable("length".to_string())
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_parse_complex_program() {
        let input = r#"
            ; Define variables
            make "x 10
            make "y 20

            ; Move to starting position
            penup
            setxy :x :y
            pendown

            ; Draw a circle
            repeat 360 [
                forward 1
                right 1
            ]

            ; Procedure with local variables
            to drawPolygon :sides :length
                repeat :sides [
                    forward :length
                    right 360 / :sides
                ]
            end

            ; Call the procedure
            drawPolygon 5 50
        "#;

        let expected = vec![
            Statement::Comment(" Define variables".to_string()),
            Statement::Make(
                "x".to_string(),
                Expression::Number(10.0),
            ),
            Statement::Make(
                "y".to_string(),
                Expression::Number(20.0),
            ),
            Statement::Comment(" Move to starting position".to_string()),
            Statement::PenUp,
            Statement::SetXY(
                Expression::Variable("x".to_string()),
                Expression::Variable("y".to_string()),
            ),
            Statement::PenDown,
            Statement::Comment(" Draw a circle".to_string()),
            Statement::Repeat(
                Expression::Number(360.0),
                vec![
                    Statement::Forward(Expression::Number(1.0)),
                    Statement::Right(Expression::Number(1.0)),
                ],
            ),
            Statement::Comment(" Procedure with local variables".to_string()),
            Statement::Procedure(
                "drawPolygon".to_string(),
                vec!["sides".to_string(), "length".to_string()],
                vec![Statement::Repeat(
                    Expression::Variable("sides".to_string()),
                    vec![
                        Statement::Forward(Expression::Variable("length".to_string())),
                        Statement::Right(Expression::BinaryOp(
                            Box::new(Expression::Number(360.0)),
                            Operator::Divide,
                            Box::new(Expression::Variable("sides".to_string())),
                        )),
                    ],
                )],
            ),
            Statement::Comment(" Call the procedure".to_string()),
            Statement::Call(
                "drawPolygon".to_string(),
                vec![
                    Expression::Number(5.0),
                    Expression::Number(50.0),
                ],
            ),
        ];

        assert_eq!(parse_program(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_while_loop() {
        let input = r#"
            make "i 0
            while :i < 10 [
                print :i
                make "i :i + 1
            ]
        "#;

        assert_eq!(
            parse_program(input),
            Ok((
                "",
                vec![
                    Statement::Make(
                        "i".to_string(),
                        Expression::Number(0.0)
                    ),
                    Statement::While(
                        Expression::BinaryOp(
                            Box::new(Expression::Variable("i".to_string())),
                            Operator::LessThan,
                            Box::new(Expression::Number(10.0))
                        ),
                        vec![
                            Statement::Call(
                                "print".to_string(),
                                vec![Expression::Variable("i".to_string())]
                            ),
                            Statement::Make(
                                "i".to_string(),
                                Expression::BinaryOp(
                                    Box::new(Expression::Variable("i".to_string())),
                                    Operator::Add,
                                    Box::new(Expression::Number(1.0))
                                )
                            ),
                        ]
                    ),
                ]
            ))
        );
    }

    #[test]
    fn test_parse_for_loop() {
        let input = r#"
            for "i from 1 to 5 [
                print :i
            ]
        "#;

        assert_eq!(
            parse_for(input),
            Ok((
                "",
                Statement::For(
                    "i".to_string(),
                    Expression::Number(1.0),
                    Expression::Number(5.0),
                    Expression::Number(1.0),
                    vec![Statement::Call(
                        "print".to_string(),
                        vec![Expression::Variable("i".to_string())]
                    )]
                )
            ))
        );

        let input = r#"
            for "angle from 0 to 360 by 15 [
                forward 100
                right :angle
            ]
        "#;

        assert_eq!(
            parse_for(input),
            Ok((
                "",
                Statement::For(
                    "angle".to_string(),
                    Expression::Number(0.0),
                    Expression::Number(360.0),
                    Expression::Number(15.0),
                    vec![
                        Statement::Forward(Expression::Number(100.0)),
                        Statement::Right(Expression::Variable("angle".to_string()))
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_parse_setxy() {
        let input = "setxy 100 200";
        assert_eq!(
            parse_setxy(input),
            Ok((
                "",
                Statement::SetXY(
                    Expression::Number(100.0),
                    Expression::Number(200.0)
                )
            ))
        );

        let input = "setxy :x :y";
        assert_eq!(
            parse_setxy(input),
            Ok((
                "",
                Statement::SetXY(
                    Expression::Variable("x".to_string()),
                    Expression::Variable("y".to_string())
                )
            ))
        );
    }

    #[test]
    fn test_parse_clearscreen() {
        let input = "clearscreen";
        assert_eq!(
            parse_clearscreen(input),
            Ok(("", Statement::ClearScreen))
        );

        let input = "cs";
        assert_eq!(
            parse_clearscreen(input),
            Ok(("", Statement::ClearScreen))
        );
    }
}
