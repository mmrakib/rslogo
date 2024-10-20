use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_while1},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, UnspecializedInput,
};

use std::collections::HashMap;
use crate::ast::{Expression, Command};

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(tuple((
            opt(tag("-")),
            digit1,
        ))),
        |s: &str| s.parse::<i32>(),
    )(input)
}

fn parse_penup(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("penup")(input)?;

    Ok((input, Command::PenUp))
}

fn parse_pendown(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("pendown")(input)?;

    Ok((input, Command::PenDown))
}

fn parse_forward(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("forward")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_integer(input)?;

    Ok((input, Command::Forward(num_pixels)))
}

fn parse_back(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("back")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_integer(input)?;

    Ok((input, Command::Back(num_pixels)))
}

fn parse_left(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("left")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_integer(input)?;

    Ok((input, Command::Left(num_pixels)))
}

fn parse_right(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("right")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_integer(input)?;

    Ok((input, Command::Right(num_pixels)))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let pen_controls_group = alt((
        parse_penup,
        parse_pendown,
    ));

    let turtle_movement_controls_group = alt((
        parse_forward,
        parse_back,
        parse_left,
        parse_right,
    ));

    alt((
        pen_controls_group,
        turtle_movement_controls_group,
    ))(input)
}

pub fn parse_program(content: String) -> Vec<Command> {
    let mut ast: Vec<Command> = Vec::new();
    let mut state = ParserState::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        match parse_command(trimmed) {
            Ok((_, command)) => ast.push(command),
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    return ast;
}
