extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, space0},
    IResult,
};

fn parse_assignment(input: &str) -> IResult<&str, String> {
    let (input, var_name) = alphanumeric1(input)?;
    let (input, _) = space0(input)?;
    let (input, operator) = alt((
        tag("+="),
        tag("-="),
        tag("*="),
        tag("/="),
        tag("%="),
        tag("..="),
        tag("||="),
    ))(input)?;
    let (input, _) = space0(input)?;
    let (input, value) = digit1(input)?;

    let transformed = match operator {
        "+=" | "-=" | "*=" | "/=" | "%=" => {
            format!("{} = {} {} {}", var_name, var_name, &operator[..1], value)
        }
        "..=" | "||=" => format!("{} = {} {} {}", var_name, var_name, &operator[..2], value),
        _ => String::from("Invalid operator"),
    };

    Ok((input, transformed))
}

fn main() {
    loop {
        println!("Enter an assignment expression:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.is_empty() {
            println!("Empty input. Exiting the program.");
            break;
        }

        match parse_assignment(input) {
            Ok((_, transformed)) => println!("Transformed expression: {}", transformed),
            Err(_) => println!("Parsing failed"),
        }
    }
}
