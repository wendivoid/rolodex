use super::parse_parameter;
use nom::{do_parse, error::VerboseError, named, opt, separated_list0, tag, IResult};

use std::fmt;

use crate::{Parameter, Parse, ParseError};

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameters<'a>(pub Vec<Parameter<'a>>);

impl<'a> fmt::Display for Parameters<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            Ok(())
        } else {
            for param in &self.0 {
                write!(f, ";{}", param)?;
            }
            Ok(())
        }
    }
}

impl<'a> Parse<'a> for Parameters<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self, ParseError> {
        match parse_parameters(input) {
            Ok((remains, item)) => Ok((remains, Parameters(item))),
            Err(nom::Err::Failure(err)) => Err(nom::Err::Failure(ParseError(err))),
            Err(nom::Err::Error(err)) => Err(nom::Err::Error(ParseError(err))),
            Err(nom::Err::Incomplete(item)) => Err(nom::Err::Incomplete(item)),
        }
    }
}

named!(pub parse_parameters<&str, Vec<Parameter>, VerboseError<&str>>, do_parse!(
    opt!(tag!(";")) >>
    data: separated_list0!(tag!(";"), parse_parameter) >>
    (data)
));
