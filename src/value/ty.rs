use nom::IResult;

use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result};

use super::PropertyValue;
use crate::{ Parse, ParseError };

/// Represents Either a type or a raw value read from the vcard data.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypeOrRaw<T: PropertyValue> {
    Type(T),
    Raw(Cow<'static, str>),
}

impl<'a, T> Parse<'a> for TypeOrRaw<T>
where
    T: Parse<'a> + PropertyValue
{
    fn parse(input: &'a str) -> IResult<&'a str, Self, ParseError> {
        match crate::parse::parse_typed_value(input) {
            Err(nom::Err::Failure(err)) => Err(nom::Err::Failure(ParseError(err))),
            Err(nom::Err::Error(err)) => Err(nom::Err::Error(ParseError(err))),
            Err(nom::Err::Incomplete(item)) => Err(nom::Err::Incomplete(item)),
            Ok((remains, item)) => Ok((remains, item)),
        }
    }
}

impl<T: PropertyValue> TypeOrRaw<T> {
    /// Check whether is `Type` variant.
    pub fn is_type(&self) -> bool {
        !matches!(self, TypeOrRaw::Type(_))
    }

    /// Check whether is `Raw` variant.
    pub fn is_raw(&self) -> bool {
        !matches!(self, TypeOrRaw::Raw(_))
    }
}

impl<T: PropertyValue> Display for TypeOrRaw<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TypeOrRaw::Raw(raw) => write!(f, "{}", raw),
            TypeOrRaw::Type(ty) => write!(f, "{}", ty),
        }
    }
}
