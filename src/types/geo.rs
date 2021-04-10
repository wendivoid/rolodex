use nom::{do_parse, named, parse_to, tag, IResult};

use std::fmt;

use crate::{Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoPosition(pub f64, pub f64);

impl fmt::Display for GeoPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl<'a> Parse<'a> for GeoPosition {
    fn parse(input: &str) -> IResult<&str, GeoPosition, ParseError> {
        parse_geo_position(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(crate::parse::to_parse_error(input, err)),
            nom::Err::Failure(err) => nom::Err::Failure(crate::parse::to_parse_error(input, err)),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_geo_position<&str, GeoPosition>, do_parse!(
    x: parse_to!(f64) >>
    tag!(",") >>
    y: parse_to!(f64) >>
    (GeoPosition(x, y))
));
