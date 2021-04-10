use nom::{do_parse, error::VerboseError, named, opt, preceded, tag, take_till, Err, IResult};

use std::borrow::Cow;
use std::fmt;

use crate::{parse::parse_name, Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct Parameter<'a> {
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub name: Cow<'a, str>,
    #[cfg_attr(
        feature = "typed-builder",
        builder(default, setter(into, strip_option))
    )]
    pub value: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}={}", self.name, value)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl<'a> Parse<'a> for Parameter<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self, ParseError> {
        match parse_parameter(input) {
            Ok(item) => Ok(item),
            Err(Err::Failure(err)) => Err(Err::Failure(ParseError(err))),
            Err(Err::Error(err)) => Err(Err::Error(ParseError(err))),
            Err(Err::Incomplete(item)) => Err(Err::Incomplete(item)),
        }
    }
}

named!(pub parse_parameter<&str, Parameter, VerboseError<&str>>, do_parse!(
    name: parse_name >>
    value: opt!(preceded!(tag!("="), parse_parameter_value)) >>
    (Parameter { name, value })
));

named!(parse_parameter_value<&str, Cow<str>, VerboseError<&str>>, do_parse!(
    value: take_till!(|x| ":;".contains(x)) >>
    (value.into())
));
