use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1, take_while_m_n},
    character::complete::{
        alphanumeric1, digit1, multispace0, multispace1, one_of
    },
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{pair, preceded, separated_pair, tuple, delimited},
};

use crate::constants::Command;

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(
        preceded(
            opt(tag("\"")),
            recognize(
                pair(
                    opt(one_of("+-")),
                    digit1,
                )
            ),
        ),
        |digit: &str| digit.parse::<i32>(),
    )(input)
}

fn parse_variable_name(input: &str) -> IResult<&str, String> {
    let (input, _) = opt(tag("\""))(input)?;
    // Parse the first character (must be a letter or underscore)
    let (input, first_char) = recognize(
        one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")
    )(input)?;
    // Parse the rest of the variable name (letters, digits, or underscores)
    let (input, rest) = take_while(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    let mut name = String::from(first_char);
    name.push_str(rest);
    Ok((input, name))
}

fn parse_value(input: &str) -> IResult<&str, String> {
    preceded(
        multispace0,
        alt((
            // Number
            map(
                recognize(pair(opt(one_of("+-")), digit1)),
                |s: &str| s.to_string(),
            ),
            // Quoted word (starts with a quote and at least one character)
            map(
                preceded(
                    tag("\""),
                    take_while1(|c: char| !c.is_whitespace())
                ),
                |s: &str| s.to_string(),
            ),
            // String literal (enclosed in double quotes)
            map(
                delimited(
                    tag("\""),
                    take_while(|c| c != '\"'),
                    tag("\"")
                ),
                |s: &str| s.to_string(),
            ),
            // Word (unquoted string without spaces or quotes)
            map(
                take_while1(|c: char| !c.is_whitespace() && c != '\"'),
                |s: &str| s.to_string(),
            ),
        )),
    )(input)
}

fn parse_penup(input: &str) -> IResult<&str, Command> {
    map(tag_no_case("PENUP"), |_| Command::PenUp)(input)
}

fn parse_pendown(input: &str) -> IResult<&str, Command> {
    map(tag_no_case("PENDOWN"), |_| Command::PenDown)(input)
}

fn parse_forward(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("FORWARD")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = parse_integer(input)?;
    Ok((input, Command::Forward(distance)))
}

fn parse_back(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("BACK")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = parse_integer(input)?;
    Ok((input, Command::Back(distance)))
}

fn parse_left(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("LEFT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, angle) = parse_integer(input)?;
    Ok((input, Command::Left(angle)))
}

fn parse_right(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("RIGHT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, angle) = parse_integer(input)?;
    Ok((input, Command::Right(angle)))
}

fn parse_setpencolor(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("SETPENCOLOR")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, color) = map_res(
        preceded(
            opt(tag("\"")),
            recognize(
                pair(
                    opt(one_of("+-")),
                    digit1,
                )
            ),
        ),
        |digit_str: &str| digit_str.parse::<u32>(),
    )(input)?;
    Ok((input, Command::SetPenColor(color)))
}

fn parse_turn(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("TURN")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, angle) = parse_integer(input)?;
    Ok((input, Command::Turn(angle)))
}

fn parse_setheading(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("SETHEADING")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, angle) = parse_integer(input)?;
    Ok((input, Command::SetHeading(angle)))
}

fn parse_setx(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("SETX")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, x) = parse_integer(input)?;
    Ok((input, Command::SetX(x)))
}

fn parse_sety(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("SETY")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, y) = parse_integer(input)?;
    Ok((input, Command::SetY(y)))
}

fn parse_make(input: &str) -> IResult<&str, Command> {
    let (input, _) = preceded(multispace0, tag_no_case("MAKE"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, var_name) = parse_variable_name(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value) = parse_value(input)?;
    let (input, _) = multispace0(input)?;

    // If there's any non-whitespace input remaining, return an error
    if !input.trim().is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::NonEmpty,
        )));
    }

    Ok((input, Command::Make(var_name, value)))
}

fn parse_addassign(input: &str) -> IResult<&str, Command> {
    let (input, _) = preceded(multispace0, tag_no_case("ADDASSIGN"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, var_name) = parse_variable_name(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value) = parse_value(input)?;
    let (input, _) = multispace0(input)?;

    // If there's any non-whitespace input remaining, return an error
    if !input.trim().is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::NonEmpty,
        )));
    }

    Ok((input, Command::AddAssign(var_name, value)))
}

fn parse_turtle_command(input: &str) -> IResult<&str, Command> {
    preceded(
        multispace0,
        alt((
            parse_penup,
            parse_pendown,
            parse_forward,
            parse_back,
            parse_left,
            parse_right,
            parse_setpencolor,
            parse_turn,
            parse_setheading,
            parse_setx,
            parse_sety,
            parse_make,
            parse_addassign,
        )),
    )(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_turtle_command)(input)
}

mod tests {
    use super::*;

    // Test parse_integer
    #[test]
    fn test_parse_integer() {
        use std::collections::HashMap;

        // Define a list of test cases
        let test_cases: HashMap<&str, Result<(i32, &str), &str>> = [
            // Input, Expected Result
            ("123", Ok((123, ""))),
            ("-456", Ok((-456, ""))),
            ("+789", Ok((789, ""))),
            ("\"321", Ok((321, ""))),
            ("\"-654", Ok((-654, ""))),
            ("0", Ok((0, ""))),
            ("\"123abc", Ok((123, "abc"))),
            ("\"", Err("Error: Expected digits after '\"'")),
            ("-", Err("Error: Expected digits after '-'")),
            ("abc", Err("Error: Expected a number")),
        ]
        .iter()
        .cloned()
        .collect();

        for (input, expected) in test_cases {
            let result = parse_integer(input);
            match expected {
                Ok((expected_value, expected_remaining)) => {
                    match result {
                        Ok((remaining, value)) => {
                            assert_eq!(value, expected_value, "Input: {}", input);
                            assert_eq!(remaining, expected_remaining, "Input: {}", input);
                        }
                        Err(err) => panic!("Input: '{}', Expected Ok, got Err: {:?}", input, err),
                    }
                }
                Err(expected_error) => {
                    assert!(
                        result.is_err(),
                        "Input: '{}', Expected error but got Ok: {:?}", input, result
                    );
                }
            }
        }
    }

    // Test parse_penup
    #[test]
    fn test_parse_penup() {
        let input = "PENUP";
        let result = parse_penup(input);
        assert_eq!(result, Ok(("", Command::PenUp)));

        let input = "penup";
        let result = parse_penup(input);
        assert_eq!(result, Ok(("", Command::PenUp)));

        let input = "PENUP extra";
        let result = parse_penup(input);
        assert_eq!(result, Ok((" extra", Command::PenUp)));

        let input = "PENUP123";
        let result = parse_penup(input);
        assert_eq!(result, Ok(("123", Command::PenUp)));
    }

    // Test parse_pendown
    #[test]
    fn test_parse_pendown() {
        let input = "PENDOWN";
        let result = parse_pendown(input);
        assert_eq!(result, Ok(("", Command::PenDown)));

        let input = "pendown";
        let result = parse_pendown(input);
        assert_eq!(result, Ok(("", Command::PenDown)));

        let input = "PENDOWN extra";
        let result = parse_pendown(input);
        assert_eq!(result, Ok((" extra", Command::PenDown)));

        let input = "PENDOWN123";
        let result = parse_pendown(input);
        assert_eq!(result, Ok(("123", Command::PenDown)));
    }

    // Test parse_forward
    #[test]
    fn test_parse_forward() {
        let input = "FORWARD 100";
        let result = parse_forward(input);
        assert_eq!(result, Ok(("", Command::Forward(100))));

        let input = "FORWARD \"-50";
        let result = parse_forward(input);
        assert_eq!(result, Ok(("", Command::Forward(-50))));

        let input = "forward +25";
        let result = parse_forward(input);
        assert_eq!(result, Ok(("", Command::Forward(25))));

        let input = "FORWARD";
        let result = parse_forward(input);
        assert!(result.is_err()); // Missing distance

        let input = "FORWARD abc";
        let result = parse_forward(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_back
    #[test]
    fn test_parse_back() {
        let input = "BACK 200";
        let result = parse_back(input);
        assert_eq!(result, Ok(("", Command::Back(200))));

        let input = "BACK \"-75";
        let result = parse_back(input);
        assert_eq!(result, Ok(("", Command::Back(-75))));

        let input = "back +50";
        let result = parse_back(input);
        assert_eq!(result, Ok(("", Command::Back(50))));

        let input = "BACK";
        let result = parse_back(input);
        assert!(result.is_err()); // Missing distance

        let input = "BACK xyz";
        let result = parse_back(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_left
    #[test]
    fn test_parse_left() {
        let input = "LEFT 90";
        let result = parse_left(input);
        assert_eq!(result, Ok(("", Command::Left(90))));

        let input = "LEFT \"-45";
        let result = parse_left(input);
        assert_eq!(result, Ok(("", Command::Left(-45))));

        let input = "left +30";
        let result = parse_left(input);
        assert_eq!(result, Ok(("", Command::Left(30))));

        let input = "LEFT";
        let result = parse_left(input);
        assert!(result.is_err()); // Missing angle

        let input = "LEFT abc";
        let result = parse_left(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_right
    #[test]
    fn test_parse_right() {
        let input = "RIGHT 180";
        let result = parse_right(input);
        assert_eq!(result, Ok(("", Command::Right(180))));

        let input = "RIGHT \"-90";
        let result = parse_right(input);
        assert_eq!(result, Ok(("", Command::Right(-90))));

        let input = "right +60";
        let result = parse_right(input);
        assert_eq!(result, Ok(("", Command::Right(60))));

        let input = "RIGHT";
        let result = parse_right(input);
        assert!(result.is_err()); // Missing angle

        let input = "RIGHT xyz";
        let result = parse_right(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_setpencolor
    #[test]
    fn test_parse_setpencolor() {
        let input = "SETPENCOLOR 14";
        let result = parse_setpencolor(input);
        assert_eq!(result, Ok(("", Command::SetPenColor(14))));

        let input = "SETPENCOLOR \"255";
        let result = parse_setpencolor(input);
        assert_eq!(result, Ok(("", Command::SetPenColor(255))));

        let input = "setpencolor 0";
        let result = parse_setpencolor(input);
        assert_eq!(result, Ok(("", Command::SetPenColor(0))));

        let input = "SETPENCOLOR -1";
        let result = parse_setpencolor(input);
        assert!(result.is_err()); // Negative color value

        let input = "SETPENCOLOR abc";
        let result = parse_setpencolor(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_turn
    #[test]
    fn test_parse_turn() {
        let input = "TURN 360";
        let result = parse_turn(input);
        assert_eq!(result, Ok(("", Command::Turn(360))));

        let input = "TURN \"-180";
        let result = parse_turn(input);
        assert_eq!(result, Ok(("", Command::Turn(-180))));

        let input = "turn +90";
        let result = parse_turn(input);
        assert_eq!(result, Ok(("", Command::Turn(90))));

        let input = "TURN";
        let result = parse_turn(input);
        assert!(result.is_err()); // Missing angle

        let input = "TURN xyz";
        let result = parse_turn(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_setheading
    #[test]
    fn test_parse_setheading() {
        let input = "SETHEADING 270";
        let result = parse_setheading(input);
        assert_eq!(result, Ok(("", Command::SetHeading(270))));

        let input = "SETHEADING \"-90";
        let result = parse_setheading(input);
        assert_eq!(result, Ok(("", Command::SetHeading(-90))));

        let input = "setheading +180";
        let result = parse_setheading(input);
        assert_eq!(result, Ok(("", Command::SetHeading(180))));

        let input = "SETHEADING";
        let result = parse_setheading(input);
        assert!(result.is_err()); // Missing angle

        let input = "SETHEADING abc";
        let result = parse_setheading(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_setx
    #[test]
    fn test_parse_setx() {
        let input = "SETX 100";
        let result = parse_setx(input);
        assert_eq!(result, Ok(("", Command::SetX(100))));

        let input = "SETX \"-50";
        let result = parse_setx(input);
        assert_eq!(result, Ok(("", Command::SetX(-50))));

        let input = "setx +75";
        let result = parse_setx(input);
        assert_eq!(result, Ok(("", Command::SetX(75))));

        let input = "SETX";
        let result = parse_setx(input);
        assert!(result.is_err()); // Missing coordinate

        let input = "SETX xyz";
        let result = parse_setx(input);
        assert!(result.is_err()); // Invalid number
    }

    // Test parse_sety
    #[test]
    fn test_parse_sety() {
        let input = "SETY 200";
        let result = parse_sety(input);
        assert_eq!(result, Ok(("", Command::SetY(200))));

        let input = "SETY \"-100";
        let result = parse_sety(input);
        assert_eq!(result, Ok(("", Command::SetY(-100))));

        let input = "sety +150";
        let result = parse_sety(input);
        assert_eq!(result, Ok(("", Command::SetY(150))));

        let input = "SETY";
        let result = parse_sety(input);
        assert!(result.is_err()); // Missing coordinate

        let input = "SETY abc";
        let result = parse_sety(input);
        assert!(result.is_err()); // Invalid number
    }

    #[test]
    fn test_parse_make() {
        let test_cases = vec![
            ("MAKE \"x 10", Command::Make("x".to_string(), "10".to_string())),
            ("make \"y -20", Command::Make("y".to_string(), "-20".to_string())),
            ("MAKE \"result \"Hello", Command::Make("result".to_string(), "Hello".to_string())),
            ("MAKE varName 123", Command::Make("varName".to_string(), "123".to_string())),
            ("MAKE \"_temp \"World", Command::Make("_temp".to_string(), "World".to_string())),
        ];

        for (input, expected_command) in test_cases {
            let result = parse_make(input);
            match result {
                Ok((remaining, command)) => {
                    assert_eq!(command, expected_command, "Input: '{}'", input);
                    assert!(remaining.trim().is_empty(), "Input: '{}', Remaining: '{}'", input, remaining);
                }
                Err(err) => panic!("Input: '{}', Expected Ok, got Err: {:?}", input, err),
            }
        }
    }

    #[test]
    fn test_parse_make_invalid() {
        let invalid_inputs = vec![
            "MAKE",                    // Missing variable and value
            "MAKE \"x",                // Missing value
            "MAKE 123 456",            // Invalid variable name starting with a digit
            "MAKE \"x \"",             // Value is empty
            "MAKE \"x invalid value",  // Invalid value (contains spaces without quotes)
        ];

        for input in invalid_inputs {
            let result = parse_make(input);
            assert!(
                result.is_err(),
                "Input: '{}', Expected error but got Ok: {:?}", input, result
            );
        }
    }

    #[test]
    fn test_parse_addassign() {
        let test_cases = vec![
            ("ADDASSIGN \"x 5", Command::AddAssign("x".to_string(), "5".to_string())),
            ("addassign \"y -15", Command::AddAssign("y".to_string(), "-15".to_string())),
            ("ADDASSIGN \"total \"100", Command::AddAssign("total".to_string(), "100".to_string())),
            ("ADDASSIGN varName 20", Command::AddAssign("varName".to_string(), "20".to_string())),
            ("ADDASSIGN \"_count \"1", Command::AddAssign("_count".to_string(), "1".to_string())),
        ];

        for (input, expected_command) in test_cases {
            let result = parse_addassign(input);
            match result {
                Ok((remaining, command)) => {
                    assert_eq!(command, expected_command, "Input: '{}'", input);
                    assert!(remaining.trim().is_empty(), "Input: '{}', Remaining: '{}'", input, remaining);
                }
                Err(err) => panic!("Input: '{}', Expected Ok, got Err: {:?}", input, err),
            }
        }
    }

    #[test]
    fn test_parse_addassign_invalid() {
        let invalid_inputs = vec![
            "ADDASSIGN",                  // Missing variable and value
            "ADDASSIGN \"x",              // Missing value
            "ADDASSIGN 123 456",          // Invalid variable name starting with a digit
            "ADDASSIGN \"x \"",           // Value is empty
            "ADDASSIGN \"x invalid value",// Invalid value (contains spaces without quotes)
        ];

        for input in invalid_inputs {
            let result = parse_addassign(input);
            assert!(
                result.is_err(),
                "Input: '{}', Expected error but got Ok: {:?}", input, result
            );
        }
    }

}
