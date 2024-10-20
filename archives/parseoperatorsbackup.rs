fn parse_addition(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_value(input)?;

    let (input, list) = many0(
        tuple((
            multispace0,   // Optional whitespace
            tag("+"),      // The '+' operator
            multispace0,   // Optional whitespace
            parse_value,  // The right-hand side expression
        ))
    )(input)?;

    // Combine the initial expression with each parsed addition
    let expr = list.into_iter().fold(init, |acc, (_, _, _, rhs)| {
        Expression::Add(Box::new(acc), Box::new(rhs))
    });

    Ok((input, expr))
}

fn parse_subtraction(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_primary(input)?;

    let (input, list) = many0(
        tuple((
            multispace0,   // Optional whitespace
            tag("-"),      // The '-' operator
            multispace0,   // Optional whitespace
            parse_primary,  // The right-hand side expression
        ))
    )(input)?;

    // Combine the initial expression with each parsed addition
    let expr = list.into_iter().fold(init, |acc, (_, _, _, rhs)| {
        Expression::Sub(Box::new(acc), Box::new(rhs))
    });

    Ok((input, expr))
}

fn parse_multiplication(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_primary(input)?;

    let (input, list) = many0(
        tuple((
            multispace0,   // Optional whitespace
            tag("*"),      // The '*' operator
            multispace0,   // Optional whitespace
            parse_primary,  // The right-hand side expression
        ))
    )(input)?;

    // Combine the initial expression with each parsed addition
    let expr = list.into_iter().fold(init, |acc, (_, _, _, rhs)| {
        Expression::Mult(Box::new(acc), Box::new(rhs))
    });

    Ok((input, expr))
}

fn parse_division(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_primary(input)?;

    let (input, list) = many0(
        tuple((
            multispace0,   // Optional whitespace
            tag("/"),      // The '/' operator
            multispace0,   // Optional whitespace
            parse_primary,  // The right-hand side expression
        ))
    )(input)?;

    // Combine the initial expression with each parsed addition
    let expr = list.into_iter().fold(init, |acc, (_, _, _, rhs)| {
        Expression::Div(Box::new(acc), Box::new(rhs))
    });

    Ok((input, expr))
}

fn parse_modulo(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_primary(input)?;

    let (input, list) = many0(
        tuple((
            multispace0,   // Optional whitespace
            tag("%"),      // The '%' operator
            multispace0,   // Optional whitespace
            parse_primary,  // The right-hand side expression
        ))
    )(input)?;

    // Combine the initial expression with each parsed addition
    let expr = list.into_iter().fold(init, |acc, (_, _, _, rhs)| {
        Expression::Mod(Box::new(acc), Box::new(rhs))
    });

    Ok((input, expr))
}

/*
0_00
0_01
1_00
1_01
1_02
1_03
1_04
1_05
1_06
1_07
1_08
1_09 (NEED ERROR CHECKING)
1_10 (NEED ERROR CHECKING)
1_11 (NEED ADDITIONAL ERROR CHECKING)
1_12 (NEED ADDITIONAL ERROR CHECKING)
1_13 (NEED ADDITIONAL ERROR CHECKING)
2_00
2_01

*/