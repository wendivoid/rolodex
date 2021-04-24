use nom::{alt, do_parse, error::VerboseError, named, opt, pair, tag, IResult};

use std::borrow::Cow;
use std::fmt;

use crate::parse::{parse_value, Parse, ParseError};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sex {
    Male,
    Female,
    Other,
    None,
    Unknown,
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sex::Male => write!(f, "M"),
            Sex::Female => write!(f, "F"),
            Sex::Other => write!(f, "O"),
            Sex::None => write!(f, "N"),
            Sex::Unknown => write!(f, "U"),
        }
    }
}

impl<'a> Parse<'a> for Sex {
    fn parse(input: &'a str) -> IResult<&'a str, Sex, ParseError> {
        parse_sex(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(err.into()),
            nom::Err::Failure(err) => nom::Err::Failure(err.into()),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_sex<&str, Sex, VerboseError<&str>>, alt!(
    tag!("M") => { |_| Sex::Male } |
    tag!("F") => { |_| Sex::Female } |
    tag!("O") => { |_| Sex::Other } |
    tag!("N") => { |_| Sex::None } |
    tag!("U") => { |_| Sex::Unknown }
));

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct Gender<'a> {
    #[cfg_attr(feature = "typed-builder", builder(setter(strip_option), default))]
    pub sex: Option<Sex>,
    #[cfg_attr(feature = "typed-builder", builder(setter(into), default))]
    pub identity: Cow<'a, str>,
}

impl<'a> fmt::Display for Gender<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sex = self
            .sex
            .map(|x| format!("{}", x))
            .unwrap_or_else(String::new);
        write!(f, "{};{}", sex, self.identity)
    }
}

impl<'a> Parse<'a> for Gender<'a> {
    fn parse(input: &'a str) -> IResult<&str, Gender<'a>, ParseError> {
        parse_gender(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(err.into()),
            nom::Err::Failure(err) => nom::Err::Failure(err.into()),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_gender<&str, Gender<'_>, VerboseError<&str>>, do_parse!(
    sex: opt!(parse_sex) >>
    identity: opt!(pair!(tag!(";"), parse_value)) >>
    (Gender { sex, identity: identity.unwrap_or_default().1 })
));
