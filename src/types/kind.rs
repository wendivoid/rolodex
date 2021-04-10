use nom::{alt, named, tag, IResult};

use std::fmt;

use crate::parse::{Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    Individual,
    Group,
    Organization,
    Location,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Individual => write!(f, "individual"),
            Kind::Group => write!(f, "group"),
            Kind::Organization => write!(f, "org"),
            Kind::Location => write!(f, "location"),
        }
    }
}

impl<'a> Parse<'a> for Kind {
    fn parse(input: &str) -> IResult<&str, Kind, ParseError> {
        parse_kind(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(crate::parse::to_parse_error(input, err)),
            nom::Err::Failure(err) => nom::Err::Failure(crate::parse::to_parse_error(input, err)),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_kind<&str, Kind>, alt!(
    tag!("individual") => { |_| Kind::Individual } |
    tag!("group") => { |_| Kind::Group } |
    tag!("org") => { |_| Kind::Organization } |
    tag!("location") => { |_| Kind::Location }
));
