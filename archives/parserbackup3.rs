use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0},
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
};

use crate::constants::Command;

// Helper function to handle optional leading and trailing whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Helper function to handle optional leading whitespace only
fn wsl<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    preceded(multispace0, inner)
}

// Parses an integer, handling optional leading whitespace and negative numbers
fn parse_integer(input: &str) -> IResult<&str, i32> {
    ws(map_res(
        wsl(recognize(pair(opt(tag("-")), digit1))),
        |s: &str| s.parse::<i32>(),
    ))(input)
}

// Parses an identifier according to Logo's naming rules
fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
        wsl(recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))),
        |s: &str| s.to_string(),
    )(input)
}

fn parse_identifier_ws(input: &str) -> IResult<&str, String> {
    map(
        ws(recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))),
        |s: &str| s.to_string(),
    )(input)
}


// Parses a value, which can be an identifier or a number
fn parse_value(input: &str) -> IResult<&str, String> {
    alt((
        map(
            wsl(recognize(pair(opt(tag("-")), digit1))),
            |s: &str| s.to_string(),
        ),
        parse_identifier,
    ))(input)
}

fn parse_value_ws(input: &str) -> IResult<&str, String> {
    ws(alt((
        map(
            recognize(pair(opt(tag("-")), digit1)),
            |s: &str| s.to_string(),
        ),
        parse_identifier_ws,
    )))(input)
}




// Parsers for basic commands

fn parse_pen_up(input: &str) -> IResult<&str, Command> {
    ws(map(ws(tag_no_case("penup")), |_| Command::PenUp))(input)
}

fn parse_pen_down(input: &str) -> IResult<&str, Command> {
    ws(map(ws(tag_no_case("pendown")), |_| Command::PenDown))(input)
}

fn parse_forward(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((ws(tag_no_case("forward")), parse_value)),
        |(_, value)| {
            if let Ok(number) = value.parse::<i32>() {
                Command::Forward(number)
            } else {
                Command::ForwardExpr(value)
            }
        },
    ))(input)
}

fn parse_back(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((ws(tag_no_case("back")), parse_value)),
        |(_, value)| {
            if let Ok(number) = value.parse::<i32>() {
                Command::Back(number)
            } else {
                Command::BackExpr(value)
            }
        },
    ))(input)
}

fn parse_left(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((ws(tag_no_case("left")), parse_value)),
        |(_, value)| {
            if let Ok(number) = value.parse::<i32>() {
                Command::Left(number)
            } else {
                Command::LeftExpr(value)
            }
        },
    ))(input)
}

fn parse_right(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((ws(tag_no_case("right")), parse_value)),
        |(_, value)| {
            if let Ok(number) = value.parse::<i32>() {
                Command::Right(number)
            } else {
                Command::RightExpr(value)
            }
        },
    ))(input)
}

fn parse_set_pen_color(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((ws(tag_no_case("setpencolor")), parse_integer)),
        |(_, value)| Command::SetPenColor(value as u32),
    ))(input)
}

// Parsers for variables

fn parse_make(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("make")),
            parse_identifier,
            parse_value,
        )),
        |(_, var_name, value)| Command::Make(var_name, value),
    ))(input)
}

fn parse_add_assign(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("addassign")),
            parse_identifier,
            parse_value,
        )),
        |(_, var_name, value)| Command::AddAssign(var_name, value),
    ))(input)
}

// Parsers for control structures

fn parse_if_eq(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("if")),
            ws(tag_no_case("eq")),
            parse_value,
            parse_value,
            delimited(ws(tag("[")), parse_commands, ws(tag("]"))),
        )),
        |(_, _, var1, var2, commands)| Command::IfEq(var1, var2, commands),
    ))(input)
}

fn parse_while_eq(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("while")),
            ws(tag_no_case("eq")),
            parse_value,
            parse_value,
            delimited(ws(tag("[")), parse_commands, ws(tag("]"))),
        )),
        |(_, _, var1, var2, commands)| Command::WhileEq(var1, var2, commands),
    ))(input)
}

fn parse_repeat(input: &str) -> IResult<&str, Command> {
    map(
        tuple((
            ws(tag_no_case("repeat")),
            parse_value, // Do not wrap with ws
            delimited(ws(tag("[")), parse_commands, ws(tag("]"))),
        )),
        |(_, count, commands)| Command::Repeat(count, commands),
    )(input)
}

// Parsers for arithmetic operators

fn parse_add(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("add")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::Add(operand1, operand2),
    ))(input)
}

fn parse_subtract(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("subtract")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::Subtract(operand1, operand2),
    ))(input)
}

fn parse_multiply(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("multiply")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::Multiply(operand1, operand2),
    ))(input)
}

fn parse_divide(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("divide")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::Divide(operand1, operand2),
    ))(input)
}

// Parsers for logical operators

fn parse_greater_than(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("greaterthan")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::GreaterThan(operand1, operand2),
    ))(input)
}

fn parse_less_than(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("lessthan")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::LessThan(operand1, operand2),
    ))(input)
}

fn parse_and(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("and")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::And(operand1, operand2),
    ))(input)
}

fn parse_or(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            ws(tag_no_case("or")),
            parse_value,
            parse_value,
        )),
        |(_, operand1, operand2)| Command::Or(operand1, operand2),
    ))(input)
}

// Parser for procedures

fn parse_to(input: &str) -> IResult<&str, Command> {
    ws(map(
        tuple((
            tag_no_case("to"),
            parse_identifier_ws,
            many0(parse_identifier_ws),
            many0(parse_command),
            ws(tag_no_case("end")),
        )),
        |(_, name, params, commands, _)| Command::To(name, params, commands),
    ))(input)
}

// Top-level command parser

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        parse_pen_up,
        parse_pen_down,
        parse_forward,
        parse_back,
        parse_left,
        parse_right,
        parse_set_pen_color,
        parse_make,
        parse_add_assign,
        parse_if_eq,
        parse_while_eq,
        parse_repeat,
        parse_add,
        parse_subtract,
        parse_multiply,
        parse_divide,
        parse_greater_than,
        parse_less_than,
        parse_and,
        parse_or,
        parse_to,
        // Add more parsers as needed
    ))(input)
}

// Parser for multiple commands

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_command)(input)
}

// Unit tests for the parsers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pen_up() {
        let inputs = ["penup", "PENUP", "  penup  "];
        for input in &inputs {
            let result = parse_pen_up(input);
            assert_eq!(result, Ok(("", Command::PenUp)));
        }
    }

    #[test]
    fn test_parse_pen_down() {
        let inputs = ["pendown", "PENDOWN", "  pendown  "];
        for input in &inputs {
            let result = parse_pen_down(input);
            assert_eq!(result, Ok(("", Command::PenDown)));
        }
    }

    #[test]
    fn test_parse_forward() {
        let inputs = [
            ("forward 100", Command::Forward(100)),
            ("FORWARD -50", Command::Forward(-50)),
            ("  forward   size  ", Command::ForwardExpr("size".to_string())),
        ];
        for (input, expected) in &inputs {
            let result = parse_forward(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_back() {
        let inputs = [
            ("back 30", Command::Back(30)),
            ("BACK -15", Command::Back(-15)),
            ("  back   distance  ", Command::BackExpr("distance".to_string())),
        ];
        for (input, expected) in &inputs {
            let result = parse_back(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_left() {
        let inputs = [
            ("left 90", Command::Left(90)),
            ("LEFT -45", Command::Left(-45)),
            ("  left   angle  ", Command::LeftExpr("angle".to_string())),
        ];
        for (input, expected) in &inputs {
            let result = parse_left(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_right() {
        let inputs = [
            ("right 90", Command::Right(90)),
            ("RIGHT -45", Command::Right(-45)),
            ("  right   angle  ", Command::RightExpr("angle".to_string())),
        ];
        for (input, expected) in &inputs {
            let result = parse_right(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_set_pen_color() {
        let inputs = [
            ("setpencolor 5", Command::SetPenColor(5)),
            ("SETPENCOLOR 10", Command::SetPenColor(10)),
            ("  setpencolor   255  ", Command::SetPenColor(255)),
        ];
        for (input, expected) in &inputs {
            let result = parse_set_pen_color(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_make() {
        let inputs = [
            ("make var1 value1", Command::Make("var1".to_string(), "value1".to_string())),
            ("MAKE counter 0", Command::Make("counter".to_string(), "0".to_string())),
            ("  make   _var   _value  ", Command::Make("_var".to_string(), "_value".to_string())),
        ];
        for (input, expected) in &inputs {
            let result = parse_make(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_add_assign() {
        let inputs = [
            ("addassign var1 value1", Command::AddAssign("var1".to_string(), "value1".to_string())),
            ("ADDASSIGN counter 1", Command::AddAssign("counter".to_string(), "1".to_string())),
            ("  addassign   _var   _value  ", Command::AddAssign("_var".to_string(), "_value".to_string())),
        ];
        for (input, expected) in &inputs {
            let result = parse_add_assign(input);
            assert_eq!(result, Ok(("", expected.clone())));
        }
    }

    #[test]
    fn test_parse_while_eq() {
        let input = "while eq counter 10 [ forward 10 addassign counter 1 ]";
        let expected_commands = vec![
            Command::Forward(10),
            Command::AddAssign("counter".to_string(), "1".to_string()),
        ];
        let expected = Command::WhileEq("counter".to_string(), "10".to_string(), expected_commands);
        let result = parse_while_eq(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_to() {
        let input = r#"
            to square size
                repeat 4 [
                    forward size
                    right 90
                ]
            end
        "#;

        let expected_commands = vec![
            Command::Repeat(
                "4".to_string(),
                vec![
                    Command::ForwardExpr("size".to_string()),
                    Command::Right(90),
                ],
            ),
        ];

        let expected = Command::To(
            "square".to_string(),
            vec!["size".to_string()],
            expected_commands,
        );

        let result = parse_to(input);
        assert!(result.is_ok());
        let (_, cmd) = result.unwrap();
        assert_eq!(cmd, expected);
    }

    #[test]
    fn test_parse_commands() {
        let input = r#"
            penup
            forward 100
            left 90
            pendown
            make counter 0
            while eq counter 5 [
                forward 50
                right 90
                addassign counter 1
            ]
        "#;

        let expected = vec![
            Command::PenUp,
            Command::Forward(100),
            Command::Left(90),
            Command::PenDown,
            Command::Make("counter".to_string(), "0".to_string()),
            Command::WhileEq(
                "counter".to_string(),
                "5".to_string(),
                vec![
                    Command::Forward(50),
                    Command::Right(90),
                    Command::AddAssign("counter".to_string(), "1".to_string()),
                ],
            ),
        ];

        let result = parse_commands(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_integer() {
        let inputs = [
            ("123", 123),
            ("-456", -456),
            ("  789  ", 789),
            ("-0", 0),
        ];
        for (input, expected) in &inputs {
            let result = parse_integer(input);
            assert_eq!(result, Ok(("", *expected)));
        }
    }

    #[test]
    fn test_parse_identifier() {
    let inputs = [
        ("var1", "var1"),
        ("_var", "_var"),
        ("variable_name", "variable_name"),
        ("  test123  ", "test123"),
    ];
    for (input, expected) in &inputs {
        let result = parse_identifier(input);
        assert_eq!(result.map(|(rem, id)| (rem.trim(), id)), Ok(("", expected.to_string())));
    }
}

    

    // Additional tests can be added here for other parsers
}
