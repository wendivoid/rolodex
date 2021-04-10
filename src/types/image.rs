use nom::{alt, do_parse, named, tag, take_until, IResult};
use nom::error::VerboseError;

use std::borrow::Cow;
use std::fmt;

use super::{parse_url, Url};
use crate::{Parse, ParseError};
use crate::parse::parse_value;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Image<'a> {
    Url(Url<'a>),
    Data {
        ty: Cow<'a, str>,
        encoding: Cow<'a, str>,
        data: Cow<'a, str>,
    },
}

impl<'a> fmt::Display for Image<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Image::Url(url) => write!(f, "{}", url),
            Image::Data { ty, encoding, data } => write!(f, "data:{};{},{}", ty, encoding, data),
        }
    }
}

impl<'a> Parse<'a> for Image<'a> {
    fn parse(input: &str) -> IResult<&str, Image<'_>, ParseError> {
        parse_image(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(err.into()),
            nom::Err::Failure(err) => nom::Err::Failure(err.into()),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_image<&str, Image<'_>, VerboseError<&str>>, alt!(
    parse_data_image |
    parse_url_image
));

named!(parse_url_image<&str, Image<'_>, VerboseError<&str>>, do_parse!(
    url: parse_url >>
    (Image::Url(url))
));

named!(parse_data_image<&str, Image<'_>, VerboseError<&str>>, do_parse!(
    tag!("data:") >>
    ty: take_until!(";") >>
    tag!(";") >>
    encoding: take_until!(",") >>
    tag!(",") >>
    data: parse_value >>
    (Image::Data { ty: ty.into(), encoding: encoding.into(), data: data.into() })
));
