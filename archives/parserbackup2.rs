#![allow(dead_code)]

use nom::{
    IResult,
    bytes::complete::tag,
    sequence::{preceded, tuple},
    character::complete::{space1, digit1, alphanumeric1},
    combinator::map_res,
    multi::{many1, separated_list1},
    branch::alt,
    sequence::delimited
};

use crate::constants::Command;

//
// Helper functions
//
pub fn parse_integer(input: &str) -> IResult<&str, i32> {
    let transform = |s: &str| s.parse::<i32>();
    map_res(digit1, transform)(input)
}

pub fn parse_variable(input: &str) -> IResult<&str, String> {
    let transform = |s: &str| Ok::<String, nom::error::Error<&str>>(s.to_string());
    map_res(alphanumeric1, transform)(input)
}

//
// Turtle controls
//
pub fn parse_penup(input: &str) -> IResult<&str, Command> {
    let (remaining, _) = tag("PENUP")(input)?;

    Ok((remaining, Command::PenUp))
}

pub fn parse_pendown(input: &str) -> IResult<&str, Command> {
    let (remaining, _) = tag("PENDOWN")(input)?;

    Ok((remaining, Command::PenDown))
}

pub fn parse_forward(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("FORWARD"), preceded(space1, parse_integer));
    let (remaining, num_pixels) = transform(input)?;

    Ok((remaining, Command::Forward(num_pixels)))
}

pub fn parse_back(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("BACK"), preceded(space1, parse_integer));
    let (remaining, num_pixels) = transform(input)?;

    Ok((remaining, Command::Back(num_pixels)))
}

pub fn parse_left(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("LEFT"), preceded(space1, parse_integer));
    let (remaining, num_pixels) = transform(input)?;

    Ok((remaining, Command::Left(num_pixels)))
}

pub fn parse_right(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("RIGHT"), preceded(space1, parse_integer));
    let (remaining, num_pixels) = transform(input)?;

    Ok((remaining, Command::Right(num_pixels)))
}

pub fn parse_setpencolor(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("SETPENCOLOR"), preceded(space1, parse_integer));
    let (remaining, color) = transform(input)?;

    Ok((remaining, Command::SetPenColor(color as u32)))
}

pub fn parse_turn(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("TURN"), preceded(space1, parse_integer));
    let (remaining, degrees) = transform(input)?;

    Ok((remaining, Command::Turn(degrees)))
}

pub fn parse_setheading(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("SETHEADING"), preceded(space1, parse_integer));
    let (remaining, degrees) = transform(input)?;

    Ok((remaining, Command::SetHeading(degrees)))
}

pub fn parse_setx(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("SETX"), preceded(space1, parse_integer));
    let (remaining, x_value) = transform(input)?;

    Ok((remaining, Command::SetX(x_value)))
}

pub fn parse_sety(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("SETY"), preceded(space1, parse_integer));
    let (remaining, y_value) = transform(input)?;

    Ok((remaining, Command::SetY(y_value)))
}

//
// Variables
//
pub fn parse_make(input: &str) -> IResult<&str, Command> {
    let mut rhs = tuple((preceded(space1, parse_variable), preceded(space1, parse_integer)));
    let mut transform = preceded(tag("MAKE"), rhs);
    let (remaining, (var, value)) = transform(input)?;

    Ok((remaining, Command::Make(var, value)))
}

//
// Unit tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let input= "123";
        let expected: i32 = 124;
        let result= parse_integer(input).unwrap().1 + 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_variable() {
        let input = "123abc456def   789ghi";
        let expected = "123abc456def";
        let result = parse_variable(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_penup() {
        let input = "PENUP";
        let expected = Command::PenUp;
        let result = parse_penup(input).unwrap().1;
        assert_eq!(result, expected);
    }
    
    fn test_parse_pendown() {
        let input = "PENDOWN";
        let expected = Command::PenDown;
        let result = parse_pendown(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_forward() {
        let input = "FORWARD 10";
        let expected = Command::Forward(10);
        let result = parse_forward(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_backward() {
        let input = "BACK 5";
        let expected = Command::Back(5);
        let result = parse_back(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_left() {
        let input = "LEFT 7";
        let expected = Command::Left(7);
        let result = parse_left(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_right() {
        let input = "RIGHT 12";
        let expected = Command::Right(12);
        let result = parse_right(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_setpencolor() {
        let input = "SETPENCOLOR 1";
        let expected = Command::SetPenColor(1);
        let result = parse_setpencolor(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_turn() {
        let input = "TURN 45";
        let expected = Command::Turn(45);
        let result = parse_turn(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_setheading() {
        let input = "SETHEADING 90";
        let expected = Command::SetHeading(90);
        let result = parse_setheading(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_setx() {
        let input = "SETX 100";
        let expected = Command::SetX(100);
        let result = parse_setx(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_sety() {
        let input = "SETY 200";
        let expected = Command::SetY(200);
        let result = parse_sety(input).unwrap().1;
        assert_eq!(result, expected);
    }
}
