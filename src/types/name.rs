use nom::{do_parse, named, opt, separated_list0, tag, IResult};

use std::borrow::Cow;
use std::fmt;

use crate::{Parse, ParseError};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct FormattedName<'a> {
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub surname: Vec<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub given: Vec<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub additional: Vec<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub prefix: Vec<Cow<'a, str>>,
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub suffix: Vec<Cow<'a, str>>,
}

impl<'a> fmt::Display for FormattedName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{}",
            self.surname.join(","),
            self.given.join(","),
            self.additional.join(","),
            self.prefix.join(","),
            self.suffix.join(",")
        )
    }
}

impl<'a> Parse<'a> for FormattedName<'a> {
    fn parse(input: &str) -> IResult<&str, FormattedName, ParseError> {
        parse_formatted_name(input).map_err(|c| match c {
            nom::Err::Error(err) => nom::Err::Error(crate::parse::to_parse_error(input, err)),
            nom::Err::Failure(err) => nom::Err::Failure(crate::parse::to_parse_error(input, err)),
            nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        })
    }
}

named!(parse_formatted_name<&str, FormattedName>, do_parse!(
    initial: opt!(parse_formatted_name_list) >>
    tag!(";") >>
    names: opt!(parse_formatted_name_list) >>
    tag!(";") >>
    family_names: opt!(parse_formatted_name_list) >>
    tag!(";") >>
    titles: opt!(parse_formatted_name_list) >>
    opt!(tag!(";")) >>
    post_nominal: opt!(parse_formatted_name_list) >>
    (FormattedName {
        surname: unwrap_vec(initial),
        given: unwrap_vec(names),
        additional: unwrap_vec(family_names),
        prefix: unwrap_vec(titles),
        suffix: unwrap_vec(post_nominal)
    })
));

fn unwrap_vec<T>(arg: Option<Vec<T>>) -> Vec<T> {
    arg.unwrap_or_else(Vec::new)
}

named!(parse_formatted_name_list<&str, Vec<Cow<'_, str>>>, do_parse!(
    value: separated_list0!(tag!(","), crate::parse::parse_formatted_value) >>
    (value)
));
