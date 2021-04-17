use nom::IResult;

use crate::{VCard, Parse, ParseError};

use std::fmt;

pub struct Vcf<'a>(pub Vec<VCard<'a>>);

impl <'a>fmt::Display for Vcf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.0.iter()
            .map(|x|format!("{}", x))
            .collect::<Vec<String>>()
            .join("\r\n");
        write!(f, "{}", data)
    }
}

impl<'a> Parse<'a> for Vcf<'a> {
    fn parse(mut input: &'a str) -> IResult<&'a str, Self, ParseError> {
        let mut vcards = vec![];
        while let Ok((remains, vcard)) = Parse::parse(input) {
            input = remains;
            vcards.push(vcard);
        }
        Ok((input, Vcf(vcards)))
    }
}
