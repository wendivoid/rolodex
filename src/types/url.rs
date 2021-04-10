use nom::character::{is_alphabetic, is_alphanumeric};
use nom::IResult;
use nom::error::VerboseError;
use nom::{do_parse, named, opt, separated_list0, tag, take_while1};

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use crate::parse::{Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct Url<'a> {
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub schema: Cow<'a, str>,
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub domain: Cow<'a, str>,
    #[cfg_attr(
        feature = "typed-builder",
        builder(default, setter(strip_option, into))
    )]
    pub path: Option<PathBuf>,
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub params: HashMap<Cow<'a, str>, Option<Cow<'a, str>>>,
}

impl<'a> fmt::Display for Url<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let params = self
            .params
            .iter()
            .map(|(key, value)| {
                if let Some(value) = value {
                    format!("?{}={}", key, value)
                } else {
                    format!("?{}", key)
                }
            })
            .collect::<Vec<String>>()
            .join("&");
        if let Some(Some(path)) = self.path.as_ref().map(|x| x.to_str()) {
            write!(f, "{}://{}{}{}", self.schema, self.domain, path, params)
        } else {
            write!(f, "{}://{}{}", self.schema, self.domain, params)
        }
    }
}

impl<'a> Parse<'a> for Url<'a> {
    fn parse(input: &str) -> IResult<&str, Url, ParseError> {
        parse_url(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(err.into()),
            nom::Err::Failure(err) => nom::Err::Failure(err.into()),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(pub parse_url<&str, Url, VerboseError<&str>>, do_parse!(
    schema: take_while1!(|x|is_alphabetic(x as u8)) >>
    tag!("://") >>
    domain: take_while1!(|x| is_alphanumeric(x as u8) || x == '.') >>
    path: opt!(parse_path) >>
    params: opt!(parse_params) >>
    (Url { schema: schema.into(), domain: domain.into(), path, params: params.unwrap_or_default() })
));

named!(parse_path<&str, PathBuf, VerboseError<&str>>, do_parse!(
    tag!("/") >>
    data: separated_list0!(tag!("/"), take_while1!(|x| is_alphanumeric(x as u8) || "._-".contains(x))) >>
    (PathBuf::from(format!("/{}", data.join("/"))))
));

named!(parse_params<&str, HashMap<Cow<'_, str>, Option<Cow<'_, str>>>, VerboseError<&str>>, do_parse!(
    tag!("?") >>
    params: separated_list0!(tag!("&"), parse_param) >>
    (params.into_iter().collect())
));

named!(parse_param<&str, (Cow<'_, str>, Option<Cow<'_, str>>), VerboseError<&str>>, do_parse!(
    name: take_while1!(|x|is_alphabetic(x as u8)) >>
    opt!(tag!("=")) >>
    value: opt!(take_while1!(|x|is_alphabetic(x as u8))) >>
    (name.into(), value.map(|x|x.into()))
));
