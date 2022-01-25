use nom::{
    bytes::complete::{tag, take_until},
    combinator::opt,
    sequence::preceded,
    IResult,
};

pub fn take_until_next<'a>(pat: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    preceded(opt(tag(pat)), take_until(pat))
}

pub fn is_ascii_whitespace(char: char) -> bool {
    char.is_ascii_whitespace()
}
