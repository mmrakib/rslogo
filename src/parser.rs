/**
 * Imports
 */
use nom::{
    branch::alt, bytes::complete::{is_not, tag, tag_no_case, take_while1}, character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, one_of, line_ending, not_line_ending}, combinator::{map, map_res, opt, recognize, all_consuming}, multi::{many0, many1, separated_list0}, sequence::{delimited, preceded, separated_pair, terminated, tuple}, Err, IResult, UnspecializedInput
};

use crate::constants::{Block, Identifier, Expression, Statement};

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
            if !remaining.trim().is_empty() {
                panic!("Unparsed input remaining: {:?}", remaining);
            }
            ast
        },
        Err(error) => {
            panic!("{:?}", error);
        }        
    }
}

/**
 * Generic parsers
 */
fn parse_all(input: &str) -> IResult<&str, Vec<Statement>, ParserError> {
    many0(parse_comment_or_statement)(input).map(|(remaining, statements)| {
        let filtered_statements = statements.into_iter().filter_map(|s| s).collect();
        (remaining, filtered_statements)
    })
}

fn parse_comment_or_statement(input: &str) -> IResult<&str, Option<Statement>, ParserError> {
    preceded(
        multispace0,
        alt((
            map(parse_comment, |_| None), 
            map(parse_statement, |statement| Some(statement)),
        ))
    )(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement, ParserError> {
    let pen_controls_group = alt((
        parse_penup,
        parse_pendown,
    ));

    terminated(alt((
        pen_controls_group,
    )), multispace0)(input)
}

/**
 * Comments
 */
fn parse_comment(input: &str) -> IResult<&str, (), ParserError> {
    map(
        preceded(
            tag("//"),
            terminated(not_line_ending, line_ending)
        ),
        |_| ()
    )(input)
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

/*
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
*/