use nom::{
    do_parse,
    error::{VerboseError, VerboseErrorKind},
    named, separated_list1, tag, take_till1, take_while1, IResult,
};

use std::borrow::Cow;
use std::fmt::Debug;

use crate::{AtLeastOne, PropertyValue, TypeOrRaw};

pub trait Parse<'a>: Sized {
    fn parse(input: &'a str) -> IResult<&'a str, Self, ParseError>;
}

/// This trait represents something the parser knows how to manage.
pub trait Parsable<'de>: PropertyValue + Parse<'de> {}

impl<'de, T> Parsable<'de> for T where T: PropertyValue + Parse<'de> {}

pub fn to_parse_error<'a>(input: &'a str, err: nom::error::Error<&'a str>) -> ParseError<'a> {
    let errors = vec![(input, nom::error::VerboseErrorKind::Nom(err.code))];
    let error = nom::error::VerboseError { errors };
    ParseError(error)
}

/// Error Returned by the parser. This is simply a wrapper around
/// `nom::error::VerboseError`
#[derive(Debug, PartialEq)]
pub struct ParseError<'a>(pub(crate) VerboseError<&'a str>);

impl<'a> From<VerboseError<&'a str>> for ParseError<'a> {
    fn from(f: VerboseError<&'a str>) -> ParseError<'a> {
        ParseError(f)
    }
}

impl<'a> ParseError<'a> {
    /// Get a pretty error message using the `nom::error::convert_error` function.
    pub fn display(self, data: &'a str) -> String {
        nom::error::convert_error(data, self.0)
    }
}

impl<'a> Parse<'a> for chrono::NaiveDate {
    fn parse(input: &'a str) -> IResult<&'a str, Self, ParseError> {
        match chrono::NaiveDate::parse_from_str(input, crate::DATE_FORMAT) {
            Ok(date) => Ok(("", date)),
            Err(_) => {
                let error = VerboseError {
                    errors: vec![(input, VerboseErrorKind::Context("Failed to parse Datetime"))],
                };
                Err(nom::Err::Error(ParseError(error)))
            }
        }
    }
}

impl<'a> Parse<'a> for std::borrow::Cow<'a, str> {
    fn parse(input: &'a str) -> IResult<&'a str, Self, ParseError> {
        Ok(("", input.into()))
    }
}

/// Parse a type that implementes `[Parse](./trait.Parse.html)`.
// TODO: Remove this call to ToString::to_string it unecessarily converts into string negating
// the usage of `Cow`.
// error[E0759]: `input` has lifetime `'de` but it needs to satisfy a `'static` lifetime requirement
//    --> libs/vcard/src/property.rs:242:19
//     |
// 236 | pub fn parse_date_value<'de, T>(input: &'de str) -> IResult<&'de str, TypeOrRaw<T>, VerboseError<&'de str>>
//     |                                        -------- this data with lifetime `'de`...
// ...
// 242 |         Err(_) => parse_value(input).map(|(a, b)|(a, TypeOrRaw::Raw(b.into())))
//     |                   ^^^^^^^^^^^^-----^                                -------- ...and is required to live as long as `'static` here
//     |                               |
//     |                               ...is captured here...
//
pub fn parse_typed_value<'de, T>(
    input: &'de str,
) -> IResult<&'de str, TypeOrRaw<T>, VerboseError<&'de str>>
where
    T: Parsable<'de>,
{
    match T::parse(&input) {
        Ok((remains, item)) => Ok((remains, TypeOrRaw::Type(item))),
        Err(_) => parse_value(input).map(|(a, b)| (a, TypeOrRaw::Raw(b.to_string().into()))),
    }
}

pub fn parse_multiple_value(
    input: &str,
) -> IResult<&str, AtLeastOne<Cow<'_, str>>, VerboseError<&str>> {
    match parse_comma_seperated_value(input) {
        Err(err) => Err(err),
        Ok((remains, mut list)) => {
            if list.len() == 1 {
                Ok((remains, AtLeastOne::Single(list.remove(0))))
            } else {
                Ok((remains, AtLeastOne::Multiple(list)))
            }
        }
    }
}

named!(parse_comma_seperated_value<&str, Vec<Cow<str>>, VerboseError<&str>>, do_parse!(
    data: separated_list1!(tag!(","), parse_until_comma) >>
    (data)
));

fn parse_until_comma<'de>(input: &'de str) -> IResult<&'de str, Cow<str>, VerboseError<&'de str>> {
    let (remains, data) = nom::bytes::complete::take_until(",")(input)?;
    Ok((remains, data.into()))
}

named!(pub(crate) parse_value<&str, Cow<str>, VerboseError<&str>>, do_parse!(
    value: take_until_unescaped_newline >>
    (value)
));

fn take_until_unescaped_newline(input: &str) -> IResult<&str, Cow<'_, str>, VerboseError<&str>> {
    let mut escaped = false;
    for (index, chr) in input.chars().enumerate() {
        if chr == '\\' {
            escaped = true;
        } else if chr == '\n' {
            if !escaped {
                return Ok((&input[index..], input[..index].into()));
            }
        } else {
            escaped = false;
        }
    }
    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}

named!(pub(crate) _parse_name<&str, Cow<str>, VerboseError<&str>>, do_parse!(
    value: take_till1!(|x| x == ':' || x == '=' || x == ';') >>
    (value.into())
));

pub(crate) fn parse_name(input: &str) -> IResult<&str, Cow<'_, str>, VerboseError<&str>> {
    let (input, data) = _parse_name(input)?;
    match data {
        Cow::Borrowed("END") => Err(nom::Err::Error(VerboseError { errors: vec![(input, VerboseErrorKind::Context("Found END:VCARD tag"))]})),
        Cow::Borrowed("BEGIN") => Err(nom::Err::Error(VerboseError { errors: vec![(input, VerboseErrorKind::Context("Found BEGIN:VCARD tag"))]})),
        data => Ok((input, data.into()))
    }
}

named!(pub(crate) parse_formatted_value<&str, Cow<'_, str>>, do_parse!(
    data: take_while1!(|x| !",;\n".contains(x)) >>
    (Cow::Borrowed(data))
));
