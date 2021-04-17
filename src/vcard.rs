use nom::{error::VerboseError, IResult};

use std::fmt;

use crate::property::{ parse_end, parse_begin };
use crate::{ Parse, ParseError, Property };

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VCard<'a>(pub Vec<Property<'a>>);

impl<'a> fmt::Display for VCard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.0.iter()
            .map(|x|format!("{}", x))
            .collect::<Vec<String>>()
            .join("\r\n");
        write!(f, "BEGIN:VCARD\r\n{}\r\nEND:VCARD", data)?;
        Ok(())
    }
}

impl<'a> VCard<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Property<'a>> {
        self.0.iter()
    }

    pub fn parse(input: &'a str) -> std::result::Result<VCard<'a>, nom::Err<ParseError>> {
        Ok(Parse::parse(input)?.1)
    }
}

impl<'a> Parse<'a> for VCard<'a> {
    fn parse(mut input: &'a str) -> IResult<&'a str, Self, ParseError> {
        use nom::bytes::complete::tag;
        let mut properties = vec![];
        match parse_begin(input) {
            Ok((remains, _)) => input = remains,
            Err(nom::Err::Incomplete(size)) => return Err(nom::Err::Incomplete(size)),
            Err(nom::Err::Error(err)) => return Err(nom::Err::Error(err.into())),
            Err(nom::Err::Failure(err)) => return Err(nom::Err::Failure(err.into()))
        }
        while let Ok((remaining, prop)) = Parse::parse(input) {
            properties.push(prop);
            match tag::<&str, &str, VerboseError<&str>>("\n")(remaining) {
                Ok((remaining, _)) => input = remaining,
                Err(_) => input = remaining,
            }
        }
        match parse_end(input) {
            Ok((remains, _)) => input = remains,
            Err(nom::Err::Incomplete(size)) => return Err(nom::Err::Incomplete(size)),
            Err(nom::Err::Error(err)) => return Err(nom::Err::Error(err.into())),
            Err(nom::Err::Failure(err)) => return Err(nom::Err::Failure(err.into()))
        }
        Ok((input, VCard(properties)))
    }
}
