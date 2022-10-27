use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::{opt, recognize},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

pub mod msi_parser;

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}
pub fn float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((opt(one_of("+-")), decimal, char('.'), opt(decimal)))),
    ))(input)
}
