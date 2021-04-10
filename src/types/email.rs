use nom::character::is_alphanumeric;
use nom::{do_parse, named, tag, take_until, take_while, IResult};

use std::borrow::Cow;
use std::fmt;

use crate::{Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct Email<'a> {
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user: Cow<'a, str>,
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub domain: Cow<'a, str>,
}

impl<'a> fmt::Display for Email<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}

impl<'a> Parse<'a> for Email<'a> {
    fn parse(input: &str) -> IResult<&str, Email, ParseError> {
        parse_email(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(crate::parse::to_parse_error(input, err)),
            nom::Err::Failure(err) => nom::Err::Failure(crate::parse::to_parse_error(input, err)),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_email<&str, Email>, do_parse!(
    user: take_until!("@") >>
    tag!("@") >>
    domain: take_while!(|x| is_alphanumeric(x as u8) || x == '.') >>
    (Email { user: user.into(), domain: domain.into() })
));
