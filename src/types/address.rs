use nom::{do_parse, named, opt, tag, IResult};

use std::borrow::Cow;
use std::fmt;

use crate::{Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct Address<'a> {
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub po_box: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub extended: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub street: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub locality: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub region: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub code: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub country: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for Address<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{};{}",
            self.po_box.as_ref().map(|x| x.as_ref()).unwrap_or(""),
            self.extended.as_ref().map(|x| x.as_ref()).unwrap_or(""),
            self.street.as_ref().map(|x| x.as_ref()).unwrap_or(""),
            self.locality.as_ref().map(|x| x.as_ref()).unwrap_or(""),
            self.region.as_ref().map(|x| x.as_ref()).unwrap_or(""),
            self.code.as_ref().map(|x| x.as_ref()).unwrap_or(""),
            self.country.as_ref().map(|x| x.as_ref()).unwrap_or("")
        )
    }
}

impl<'a> Parse<'a> for Address<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Address<'a>, ParseError> {
        parse_formatted_address(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(crate::parse::to_parse_error(input, err)),
            nom::Err::Failure(err) => nom::Err::Failure(crate::parse::to_parse_error(input, err)),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_formatted_address<&str, Address<'_>>, do_parse!(
    po_box: opt!(crate::parse::parse_formatted_value) >>
    tag!(";") >>
    extended: opt!(crate::parse::parse_formatted_value) >>
    tag!(";") >>
    street: opt!(crate::parse::parse_formatted_value) >>
    tag!(";") >>
    locality: opt!(crate::parse::parse_formatted_value) >>
    tag!(";") >>
    region: opt!(crate::parse::parse_formatted_value) >>
    tag!(";") >>
    code: opt!(crate::parse::parse_formatted_value) >>
    tag!(";") >>
    country: opt!(crate::parse::parse_formatted_value) >>
    (Address { po_box, extended, street, locality, region, code, country })
));
