use nom::{
    branch::alt, bytes::complete::{is_not, tag, tag_no_case, take_while1}, character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, one_of}, combinator::{map, map_res, opt, recognize}, multi::{many0, many1, separated_list0}, sequence::{delimited, preceded, separated_pair, terminated, tuple}, Err, IResult, UnspecializedInput
};

use std::collections::HashMap;
use crate::ast::{Expression, Command};

fn parse_integer(input: &str) -> IResult<&str, Expression> {
    let (input, _) = tag("\"")(input)?;

    map_res(
        recognize(tuple((
            opt(tag("-")),
            digit1,
        ))),
        |s: &str| s.parse::<i32>().map(Expression::IntegerLiteral),
    )(input)
}

fn parse_variable(input: &str) -> IResult<&str, Expression> {
    let (input, _) = tag(":")(input)?;

    map_res(
        recognize(alphanumeric1),
        |s: &str| Ok::<Expression, &str>(Expression::Variable(s.to_string())),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_parentheses,
        parse_integer,
        parse_variable,
    ))(input)
}

fn parse_parentheses(input: &str) -> IResult<&str, Expression> {
    delimited(
        tag("("),
        parse_expression,
        tag(")")
    )(input)
}

fn parse_binary_ops(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_value(input)?;

    let (input, list) = many0(
        tuple((
            multispace0,
            alt((
                tag("&&"),
                tag("||"),
                tag("!="),
                tag("=="),
                tag(">"),
                tag("<"),
                tag("%"),
                tag("/"),
                tag("*"),
                tag("-"),
                tag("+"),
            )),
            multispace0,
            parse_value,
        ))
    )(input)?;

    let expr = list.into_iter().fold(init,
        |acc, (_, op, _, rhs)| {
            match op {
                "&&" => Expression::And(Box::new(acc), Box::new(rhs)),
                "||" => Expression::Or(Box::new(acc), Box::new(rhs)),
                "!=" => Expression::NotEquals(Box::new(acc), Box::new(rhs)),
                "==" => Expression::Equals(Box::new(acc), Box::new(rhs)),
                ">" => Expression::GreaterThan(Box::new(acc), Box::new(rhs)),
                "<" => Expression::LessThan(Box::new(acc), Box::new(rhs)),
                "%" => Expression::Mod(Box::new(acc), Box::new(rhs)),
                "/" => Expression::Div(Box::new(acc), Box::new(rhs)),
                "*" => Expression::Mult(Box::new(acc), Box::new(rhs)),
                "-" => Expression::Sub(Box::new(acc), Box::new(rhs)),
                "+" => Expression::Add(Box::new(acc), Box::new(rhs)),
                _ => unreachable!(),
            }
    });

    Ok((input, expr))
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_parentheses,
        parse_binary_ops,
        parse_value,
    ))(input)
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
    let (input, num_pixels) = parse_expression(input)?;

    Ok((input, Command::Forward(num_pixels)))
}

fn parse_back(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("back")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_expression(input)?;

    Ok((input, Command::Back(num_pixels)))
}

fn parse_left(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("left")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_expression(input)?;

    Ok((input, Command::Left(num_pixels)))
}

fn parse_right(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("right")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, num_pixels) = parse_expression(input)?;

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
