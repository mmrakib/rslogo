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

pub fn parse_backward(input: &str) -> IResult<&str, Command> {
    let mut transform = preceded(tag("BACK"), preceded(space1, parse_integer));
    let (remaining, num_pixels) = transform(input)?;

    Ok((remaining, Command::Back(num_pixels)))
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
        let result = parse_backward(input).unwrap().1;
        assert_eq!(result, expected);
    }
}
