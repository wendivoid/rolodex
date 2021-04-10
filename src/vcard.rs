use nom::{error::VerboseError, IResult};

use std::fmt;

use crate::{ Parse, ParseError, Property };

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VCard<'a>(pub Vec<Property<'a>>);

impl<'a> fmt::Display for VCard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, prop) in self.0.iter().enumerate() {
            if index == self.0.len() - 1 {
                write!(f, "{}", prop)?;
            } else {
                writeln!(f, "{}", prop)?;
            }
        }
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
        while let Ok((remaining, prop)) = Parse::parse(input) {
            properties.push(prop);
            match tag::<&str, &str, VerboseError<&str>>("\n")(remaining) {
                Ok((remaining, _)) => input = remaining,
                Err(_) => input = remaining,
            }
        }
        Ok((input, VCard(properties)))
    }
}
