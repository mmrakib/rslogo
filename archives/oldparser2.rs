/**
 * Imports
 */
use nom::{
    branch::alt, 
    bytes::complete::{is_not, tag, tag_no_case, take_while1, take_until},
    character::complete::{alpha1, alphanumeric1, digit1, line_ending, multispace0, multispace1, not_line_ending, one_of, char, newline},
    combinator::{all_consuming, map, map_res, opt, recognize, peek},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{constants::{Block, Expression, Identifier, Statement}, turtle};
use crate::error::print_error;

use std::any::type_name;

/**
 * Type aliases
 */
type ParserError<'a> = nom::error::VerboseError<&'a str>;

/**
 * Public interface
 */
pub fn parse_program(content: String) -> Block {
    let input: &str = &content;

    match parse_all(input) {
        Ok((remaining, ast)) => {
            /*
            if !remaining.trim().is_empty() {
                // TODO: Better error handling
                print_error(
                    "unparsed input remaining", 
                    &format!("Failed to parse the entirety of the input. The following string remains: {:?}", remaining),
                    &["Check the syntax/format of the input file"],
                    true,
                );
            }
            */
            ast
        },
        Err(error) => {
            // TODO: Better error handling
            panic!("{:?}", error);
        }        
    }
}

/**
 * Generic
 */
fn parse_all(input: &str) -> IResult<&str, Block, ParserError> {
    many0(
        preceded(
            multispace0,
            alt((
                map(parse_comment, |_| None),
                map(parse_statement, |statement| Some(statement)),
            ))
        )
    )(input).map(|(remaining, statements)| {
        let filtered = statements
            .into_iter()
            .filter_map(|s| s)
            .collect();

        (remaining, filtered)
    })
}

fn parse_arguments(input: &str) -> IResult<&str, Vec<Expression>, ParserError> {
    many0(
        preceded(
            multispace1,
            parse_expression
        )
    )(input)
}

/**
 * Comments
 */
fn parse_comment(input: &str) -> IResult<&str, (), ParserError> {
    preceded(
        tag("//"),
        map(terminated(not_line_ending, line_ending), |_| ())
    )(input)
}

/**
 * Identifiers
 */
fn parse_identifier(input: &str) -> IResult<&str, Identifier, ParserError> {
    let (input, prefix) = opt(alt( (tag("\""), tag(":") )))(input)?;
    let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;

    let access_modifier = match prefix {
        Some(val) => val,
        None => {
            let name_str: &str = &(name.to_lowercase());
            match name_str {
                "xcor" | "ycor" | "heading" | "color" => "Q",
                _ => "",
            }
        },
    };

    Ok((input, Identifier(name.to_string(), access_modifier.to_string())))
}

/**
 * Values
 */
fn parse_integer(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("\"")(input)?;
    let (input, sign) = opt(char('-'))(input)?;
    let (input, digits) = digit1(input)?;

    let number_str = match sign {
        Some(_) => format!("-{}", digits), // Add the minus sign if present
        None => digits.to_string(),
    };

    // Attempt to parse the combined string as an integer
    match number_str.parse::<i32>() {
        Ok(value) => Ok((input, Expression::IntegerLiteral(value))),
        // TODO: Better error handling
        Err(_) => panic!("Failed to parse integer"),
    }
}

fn parse_string(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("\"")(input)?;
    let (input, content) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;

    Ok((input, Expression::StringLiteral(content.to_string())))
}

fn parse_variable(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag(":")(input)?;
    let (input, name) = alphanumeric1(input)?;

    Ok((input, Expression::VariableReference(name.to_string())))
}

/**
 * Queries
 */
fn parse_xcor(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag_no_case("xcor")(input)?;

    Ok((input, Expression::QueryXCor))
}

fn parse_ycor(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag_no_case("ycor")(input)?;

    Ok((input, Expression::QueryYCor))
}

fn parse_heading(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag_no_case("heading")(input)?;

    Ok((input, Expression::QueryHeading))
}

fn parse_color(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag_no_case("color")(input)?;

    Ok((input, Expression::QueryColor))
}

fn parse_queries(input: &str) -> IResult<&str, Expression, ParserError> {
    alt((
        parse_xcor,
        parse_ycor,
        parse_heading,
        parse_color,
    ))(input)
}

/**
 * Expressions
 */
fn parse_value(input: &str) -> IResult<&str, Expression, ParserError> {
    alt((
        parse_parentheses,
        parse_queries,
        parse_integer,
        parse_variable,
        parse_string,
    ))(input)
}

fn parse_parentheses(input: &str) -> IResult<&str, Expression, ParserError> {
    delimited(
        tag("("),
        parse_expression,
        tag(")")
    )(input)
}

/**
 * Binary operations
 */
fn parse_binary_ops(input: &str) -> IResult<&str, Expression, ParserError> {
    alt((
        parse_addition,
        parse_subtraction,
        parse_multiplication,
        parse_division,
        parse_modulo,
        parse_equals,
        parse_not_equals,
        parse_greater_than,
        parse_less_than,
        parse_and,
        parse_or,
    ))(input)
}

fn parse_addition(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("+")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Addition(Box::new(left), Box::new(right))))
}

fn parse_subtraction(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("-")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Subtraction(Box::new(left), Box::new(right))))
}

fn parse_multiplication(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("*")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Multiplication(Box::new(left), Box::new(right))))
}

fn parse_division(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("/")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Division(Box::new(left), Box::new(right))))
}

fn parse_modulo(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("%")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Modulo(Box::new(left), Box::new(right))))
}

fn parse_equals(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("EQ")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Equals(Box::new(left), Box::new(right))))
}

fn parse_not_equals(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("NE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::NotEquals(Box::new(left), Box::new(right))))
}

fn parse_greater_than(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("GT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::GreaterThan(Box::new(left), Box::new(right))))
}

fn parse_less_than(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("LT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::LessThan(Box::new(left), Box::new(right))))
}

fn parse_and(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("AND")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::And(Box::new(left), Box::new(right))))
}

fn parse_or(input: &str) -> IResult<&str, Expression, ParserError> {
    let (input, _) = tag("OR")(input)?;
    let (input,_) = multispace1(input)?;
    let (input, (left, _, right)) = tuple((parse_expression, multispace1, parse_expression))(input)?;

    Ok((input, Expression::Or(Box::new(left), Box::new(right))))
}

fn parse_expression(input: &str) -> IResult<&str, Expression, ParserError> {
    alt((
        parse_binary_ops,
        parse_value,
    ))(input)
}

/**
 * Error handling
 */
fn check_type<T>(_: &T) -> &str {
    type_name::<T>()
}

fn check_keywords(input: &str) -> IResult<&str, &str, ParserError> {
    let pen_controls_group = alt((
        tag_no_case("penup"),
        tag_no_case("pendown"),
    ));

    let turtle_movement_group = alt((
        tag_no_case("forward"),
        tag_no_case("back"),
        tag_no_case("left"),
        tag_no_case("right"),
        tag_no_case("turn"),
    ));

    let setters_group = alt((
        tag_no_case("setx"),
        tag_no_case("sety"),
        tag_no_case("setheading"),
        tag_no_case("setpencolor"),
    ));

    let variable_assignment_group = alt((
        tag_no_case("make"),
        tag_no_case("addassign"),
    ));

    let control_structures_group = alt((
        tag_no_case("if"),
        tag_no_case("while"),
        tag_no_case("repeat"),
    ));

    let (input, keyword) = alt((
        pen_controls_group,
        turtle_movement_group,
        setters_group,
        variable_assignment_group,
        control_structures_group,
    ))(input)?;

    Ok((input, keyword))
}

fn check_error_argument_count(input: &str) -> IResult<&str, (), ParserError> {
    let (input, (keyword, args)) = peek(tuple((
        check_keywords,
        parse_arguments,
    )))(input)?;

    let keyword_str: &str = &(keyword.to_lowercase());
    match keyword_str {
        // Zero arguments
        "penup" | "pendown" => {
            if !args.is_empty() {
                print_error(
                    "invalid argument count",
                    &format!("The statement {} does not accept any arguments\n    The arguments given were: {:?}", keyword, args),
                    &["Ensure no arguments are provided"],
                    true,
                );
            }
        },
        "forward" | "back" | "left" | "right" | "turn" | "setx" | "sety" | "setheading" | "setpencolor" => {
            if args.len() != 1 {
                print_error(
                    "invalid argument count",
                    &format!("The statement {} requires exactly 1 argument\n    The arguments given were: {:?}", keyword, args),
                    &["Ensure exactly 1 argument is provided"],
                    true,
                );
            }
        },
        "make" | "addassign" => {
            if args.len() != 2 {
                print_error(
                    "invalid argument count",
                    &format!("The statement {} requires exactly 2 arguments\n    The arguments given were: {:?}", keyword, args),
                    &["Ensure exactly 2 arguments are provided"],
                    true,
                );
            }
        },
        "if" | "while" | "repeat" => {
            if args.len() >= 1 {
                println!("args: {:?}", args);
                print_error(
                    "invalid argument count",
                    &format!("The statement {} requires exactly 1 argument\n    The arguments given were: {:?}\nInput is: {:?}", keyword, args, input),
                    &["Ensure exactly 1 argument is provided"],
                    true,
                );
            }
        },
        _ => (),
    }

    Ok((input, ()))
}

fn check_error_argument_type(input: &str) -> IResult<&str, (), ParserError> {
    let (input, (keyword, args)) = peek(tuple((
        check_keywords,
        parse_arguments,
    )))(input)?;

    let keyword_str: &str = &(keyword.to_lowercase());

    match keyword_str {
        "forward" | "back" | "left" | "right" | "turn" | "setx" | "sety" | "setheading" | "setpencolor" => {
            let arg = &args[0];

            let arg_type = check_type(arg);

            match arg_type {
                "rslogo::constants::Expression" => {
                    match arg {
                        Expression::StringLiteral(_) | Expression::QueryHeading | Expression::QueryColor => {
                            print_error(
                                "invalid argument type",
                                &format!("The statement {} requires an integer expression argument\n    The argument given was: {:?}", keyword, arg),
                                &["Ensure the argument is an integer expression"],
                                true,
                            );
                        },
                        _ => (),
                    }
                },
                _ => {
                    print_error(
                        "invalid argument type",
                        &format!("The statement {} requires an integer expression argument\n    The argument given was: {:?}", keyword, arg),
                        &["Ensure the argument is an integer expression"],
                        true,
                    );
                }
            }
        },
        "make" | "addassign" => {
            let arg1 = &args[0];
            let arg2 = &args[1];

            let arg1_type = check_type(arg1);
            let arg2_type = check_type(arg2);

            match arg1_type {
                "rslogo::constants::Identifier" => (),
                "rslogo::constants::Expression" => {
                    match arg1 {
                        Expression::StringLiteral(_) | Expression::VariableReference(_) | Expression::QueryHeading | Expression::QueryColor => (),
                        _ => {
                            print_error(
                                "invalid argument type",
                                &format!("The statement {} requires an identifier as the first argument\n    The argument given was: {:?}", keyword, arg1),
                                &["Ensure the first argument is an identifier"],
                                true,
                            );
                        },
                    }
                },
                _ => {
                    print_error(
                        "invalid argument type",
                        &format!("The statement {} requires an identifier as the first argument\n    The argument given was: {:?}", keyword, arg1),
                        &["Ensure the first argument is an identifier"],
                        true,
                    );
                }
            }

            match arg2_type {
                "rslogo::constants::Expression" => (),
                _ => {
                    print_error(
                        "invalid argument type",
                        &format!("The statement {} requires an expression as the second argument\n    The argument given was: {:?}", keyword, arg2),
                        &["Ensure the second argument is an expression"],
                        true,
                    );
                }
            }
        },
        "if" | "while" | "repeat" => {
            let arg = &args[0];

            let arg_type = check_type(arg);

            match arg_type {
                "rslogo::constants::Expression" => {
                    match arg {
                        Expression::StringLiteral(_) | Expression::VariableReference(_) | Expression::QueryHeading | Expression::QueryColor => {
                            print_error(
                                "invalid argument type",
                                &format!("The statement {} requires a boolean expression argument\n    The argument given was: {:?}", keyword, arg),
                                &["Ensure the argument is a boolean expression"],
                                true,
                            );
                        },
                        _ => (),
                    }
                },
                _ => {
                    print_error(
                        "invalid argument type",
                        &format!("The statement {} requires a boolean expression argument\n    The argument given was: {:?}", keyword, arg),
                        &["Ensure the argument is a boolean expression"],
                        true,
                    );
                }
            }
        },
        _ => {
            // Do nothing
        }
    }

    Ok((input, ()))
}

/**
 * Statements
 */
fn parse_statement(input: &str) -> IResult<&str, Statement, ParserError> {
    let mut pen_controls_group = alt((
        parse_penup,
        parse_pendown,
    ));

    let mut turtle_movement_group = alt((
        parse_forward,
        parse_back,
        parse_left,
        parse_right,
        parse_turn,
    ));

    let mut setters_group = alt((
        parse_setx,
        parse_sety,
        parse_setheading,
        parse_setpencolor,
    ));

    let mut variable_assignment_group = alt((
        parse_make,
        parse_addassign,
    ));

    let mut control_structures_group = alt((
        parse_if,
        parse_while,
        parse_repeat,
    ));

    let mut procedure_group = alt((
        parse_procedure_definition,
        parse_procedure_call,
    ));

    let (_, _) = check_error_argument_count(input)?;
    let (_, _) = check_error_argument_type(input)?;

    terminated(alt((
        pen_controls_group,
        turtle_movement_group,
        setters_group,
        variable_assignment_group,
        control_structures_group,
        procedure_group,
    )), multispace0)(input)
}

/**
 * Pen control
 */
fn parse_penup(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("penup")(input)?;

    Ok((input, Statement::PenUp))
}

fn parse_pendown(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("pendown")(input)?;

    Ok((input, Statement::PenDown))
}

/**
 * Turtle movement control
 */
fn parse_forward(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("forward")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, pixels) = parse_expression(input)?;

    Ok((input, Statement::Forward( Box::new(pixels) )))
}

fn parse_back(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("back")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, pixels) = parse_expression(input)?;

    Ok((input, Statement::Back( Box::new(pixels) )))
}

fn parse_left(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("left")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, degrees) = parse_expression(input)?;

    Ok((input, Statement::Left( Box::new(degrees) )))
}

fn parse_right(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("right")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, degrees) = parse_expression(input)?;

    Ok((input, Statement::Right( Box::new(degrees) )))
}

fn parse_turn(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("turn")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, degrees) = parse_expression(input)?;

    Ok((input, Statement::Turn( Box::new(degrees) )))
}

/**
 * Setters
 */
fn parse_setx(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("setx")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, position) = parse_expression(input)?;

    Ok((input, Statement::SetX( Box::new(position) )))
}

fn parse_sety(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("sety")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, position) = parse_expression(input)?;

    Ok((input, Statement::SetY( Box::new(position) )))
}

fn parse_setheading(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("setheading")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, degrees) = parse_expression(input)?;

    Ok((input, Statement::SetHeading( Box::new(degrees) )))
}

fn parse_setpencolor(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("setpencolor")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, color) = parse_expression(input)?;

    Ok((input, Statement::SetPenColor( Box::new(color) )))
}

/**
 * Variable assignments
 */
fn parse_make(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("make")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, variable_name) = parse_identifier(input)?;
    let (input, _) = multispace1(input)?;
    let (input, variable_val) = parse_expression(input)?;

    Ok((input, Statement::Make(variable_name, Box::new(variable_val))))
}

fn parse_addassign(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("addassign")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, variable_name) = parse_identifier(input)?;
    let (input, _) = multispace1(input)?;
    let (input, variable_val) = parse_expression(input)?;

    Ok((input, Statement::AddAssign(variable_name, Box::new(variable_val))))
}

/**
 * Control structures
 */
fn parse_block(input: &str) -> IResult<&str, Block, ParserError> {
    delimited(
        tag("["),
        many0(
            preceded(
                multispace0,
                parse_statement
            )
        ),
        preceded(
            multispace0,
            tag("]")
        ),
    )(input)
}

fn parse_if(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("if")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, block) = parse_block(input)?;

    Ok((input, Statement::If( Box::new(condition), Box::new(block) )))
}

fn parse_while(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("while")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, block) = parse_block(input)?;

    Ok((input, Statement::While( Box::new(condition), Box::new(block) )))
}

fn parse_repeat(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("repeat")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, block) = parse_block(input)?;

    Ok((input, Statement::Repeat( Box::new(condition), Box::new(block) )))
}

/**
 * Procedures
 */
fn parse_procedure_definition(input: &str) -> IResult<&str, Statement, ParserError> {
    let (input, _) = tag_no_case("to")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, identifier) = parse_identifier(input)?;

    let (input, parameters_string) = not_line_ending(input)?;
    println!("proc_def parameters_string: {:?}", parameters_string);
    let (_, parameters) = parse_arguments(parameters_string)?;

    let (input, _) = multispace0(input)?;
    let (input, body_string) = take_until("END\n")(input)?;
    println!("proc_def body_string: {:?}", body_string);
    let (_, filtered) = parse_all(body_string.trim())?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag_no_case("end")(input)?;

    Ok((
        input,
        Statement::ProcedureDefinition {
            name: identifier,
            parameters: parameters,
            body: filtered,
        }
    ))
}

fn parse_procedure_call(input: &str) -> IResult<&str, Statement, ParserError> {
    // Parse the procedure name
    let (input, identifier) = parse_identifier(input)?;

    let (input, parameters_string) = not_line_ending(input)?;
    println!("proc_call parameters_string: {:?}", parameters_string);
    let (_, parameters) = parse_arguments(parameters_string)?;

    Ok((
        input,
        Statement::ProcedureCall {
            name: identifier,
            arguments: parameters,
        }
    ))
}

/** 
 * Unit Tests
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Comments
     */
    #[test]
    fn test_parse_comment() {
        let input = "// This is a comment\n";
        let expected = ();
        let result = parse_comment(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Identifiers
     */
    #[test]
    fn test_parse_identifier() {
        let input = "\"foo";
        let expected = Identifier("foo".to_string(), "\"".to_string());
        let result = parse_identifier(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Values
     */
    #[test]
    fn test_parse_integer() {
        let input = "\"123";
        let expected = Expression::IntegerLiteral(123);
        let result = parse_integer(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello";
        let expected = Expression::StringLiteral("hello".to_string());
        let result = parse_string(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_variable() {
        let input = ":foo";
        let expected = Expression::VariableReference("foo".to_string());
        let result = parse_variable(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Queries
     */
    #[test]
    fn test_parse_xcor() {
        let input = "XCOR";
        let expected = Expression::QueryXCor;
        let result = parse_xcor(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_ycor() {
        let input = "YCOR";
        let expected = Expression::QueryYCor;
        let result = parse_ycor(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_heading() {
        let input = "HEADING";
        let expected = Expression::QueryHeading;
        let result = parse_heading(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_color() {
        let input = "COLOR";
        let expected = Expression::QueryColor;
        let result = parse_color(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Parentheses
     */
    #[test]
    fn test_parse_parentheses() {
        let input = "(\"10)";
        let expected = Expression::IntegerLiteral(10);
        let result = parse_parentheses(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Operations
     */
    #[test]
    fn test_parse_addition() {
        let input = "+ \"10 \"20";
        let expected = Expression::Addition(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_subtraction() {
        let input = "- \"10 \"20";
        let expected = Expression::Subtraction(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_multiplication() {
        let input = "* \"10 \"20";
        let expected = Expression::Multiplication(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_division() {
        let input = "/ \"10 \"20";
        let expected = Expression::Division(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_modulo() {
        let input = "% \"10 \"20";
        let expected = Expression::Modulo(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_equals() {
        let input = "EQ \"10 \"20";
        let expected = Expression::Equals(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_not_equals() {
        let input = "NE \"10 \"20";
        let expected = Expression::NotEquals(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_greater_than() {
        let input = "GT \"10 \"20";
        let expected = Expression::GreaterThan(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_less_than() {
        let input = "LT \"10 \"20";
        let expected = Expression::LessThan(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_and() {
        let input = "AND \"10 \"20";
        let expected = Expression::And(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_or() {
        let input = "OR \"10 \"20";
        let expected = Expression::Or(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(Expression::IntegerLiteral(20)),
        );
        let result = parse_binary_ops(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Pen control
     */
    #[test]
    fn test_parse_penup() {
        let input = "PENUP";
        let expected = Statement::PenUp;
        let result = parse_penup(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_pendown() {
        let input = "PENDOWN";
        let expected = Statement::PenDown;
        let result = parse_pendown(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Turtle movement control
     */
    #[test]
    fn test_parse_forward() {
        let input = "FORWARD \"10";
        let expected = Statement::Forward(Box::new(Expression::IntegerLiteral(10)));
        let result = parse_forward(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_back() {
        let input = "BACK \"10";
        let expected = Statement::Back(Box::new(Expression::IntegerLiteral(10)));
        let result = parse_back(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_left() {
        let input = "LEFT \"90";
        let expected = Statement::Left(Box::new(Expression::IntegerLiteral(90)));
        let result = parse_left(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_right() {
        let input = "RIGHT \"90";
        let expected = Statement::Right(Box::new(Expression::IntegerLiteral(90)));
        let result = parse_right(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_turn() {
        let input = "TURN \"90";
        let expected = Statement::Turn(Box::new(Expression::IntegerLiteral(90)));
        let result = parse_turn(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Setters
     */
    #[test]
    fn test_parse_setx() {
        let input = "SETX \"10";
        let expected = Statement::SetX(Box::new(Expression::IntegerLiteral(10)));
        let result = parse_setx(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_sety() {
        let input = "SETY \"10";
        let expected = Statement::SetY(Box::new(Expression::IntegerLiteral(10)));
        let result = parse_sety(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_setheading() {
        let input = "SETHEADING \"90";
        let expected = Statement::SetHeading(Box::new(Expression::IntegerLiteral(90)));
        let result = parse_setheading(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_setpencolor() {
        let input = "SETPENCOLOR \"10";
        let expected = Statement::SetPenColor(Box::new(Expression::IntegerLiteral(10)));
        let result = parse_setpencolor(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Variable assignment
     */
    #[test]
    fn test_parse_make() {
        let input = "MAKE \"foo \"10";
        let expected = Statement::Make(Identifier("foo".to_string(), "\"".to_string()), Box::new(Expression::IntegerLiteral(10)));
        let result = parse_make(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_addassign() {
        let input = "ADDASSIGN \"foo \"10";
        let expected = Statement::AddAssign(Identifier("foo".to_string(), "\"".to_string()), Box::new(Expression::IntegerLiteral(10)));
        let result = parse_addassign(input);

        assert_eq!(result, Ok(("", expected)));
    }

    /**
     * Control structures
     */
    #[test]
    fn test_parse_if() {
        let input = "IF EQ \"10 \"20 [PENUP]";
        let expected = Statement::If(
            Box::new(Expression::Equals(
                Box::new(Expression::IntegerLiteral(10)),
                Box::new(Expression::IntegerLiteral(20)),
            )),
            Box::new(vec![Statement::PenUp]),
        );
        let result = parse_if(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_while() {
        let input = "WHILE EQ \"10 \"20 [PENUP]";
        let expected = Statement::While(
            Box::new(Expression::Equals(
                Box::new(Expression::IntegerLiteral(10)),
                Box::new(Expression::IntegerLiteral(20)),
            )),
            Box::new(vec![Statement::PenUp]),
        );
        let result = parse_while(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_repeat() {
        let input = "REPEAT \"10 [PENUP]";
        let expected = Statement::Repeat(
            Box::new(Expression::IntegerLiteral(10)),
            Box::new(vec![Statement::PenUp]),
        );
        let result = parse_repeat(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_procedure_definition() {
        let input = "TO Foo :bar \"baz\nPENUP\nEND\n";
        let expected = Statement::ProcedureDefinition {
            name: Identifier("Foo".to_string(), "".to_string()),
            parameters: vec![
                Expression::VariableReference("bar".to_string()),
                Expression::StringLiteral("baz".to_string()),
            ],
            body: vec![Statement::PenUp],
        };
        let result = parse_procedure_definition(input);

        assert_eq!(result, Ok(("\n", expected)));
    }

    #[test]
    fn test_parse_procedure_call() {
        let input = "Bar \"10 \"20\n";
        let expected = Statement::ProcedureCall {
            name: Identifier("Bar".to_string(), "".to_string()),
            arguments: vec![
                Expression::IntegerLiteral(10),
                Expression::IntegerLiteral(20),
            ],
        };
        let result = parse_procedure_call(input);

        assert_eq!(result, Ok(("\n", expected)));
    }
}