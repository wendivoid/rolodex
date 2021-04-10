use std::fmt::{self, Debug};

mod parameter;
pub use self::parameter::{parse_parameter, Parameter};

mod parameters;
pub use self::parameters::{parse_parameters, Parameters};

use crate::Value;

mod parse;
pub use self::parse::parse_property;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property<'a> {
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub params: Parameters<'a>,
    pub value: Value<'a>,
}

impl<'a> From<Value<'a>> for Property<'a> {
    fn from(value: Value<'a>) -> Property<'a> {
        Property {
            value,
            params: Default::default(),
        }
    }
}

impl<'a, 'b> From<&'b Value<'a>> for Property<'a> {
    fn from(value: &'b Value<'a>) -> Property<'a> {
        Property {
            value: value.clone(),
            params: Default::default(),
        }
    }
}

impl<'a> fmt::Display for Property<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Value::Begin => write!(f, "BEGIN{}:VCARD", self.params),
            Value::End => write!(f, "END{}:VCARD", self.params),
            Value::Source(src) => write!(f, "SOURCE{}:{}", self.params, src),
            Value::Kind(src) => write!(f, "KIND{}:{}", self.params, src),
            Value::Xml(xml) => write!(f, "XML{}:{}", self.params, xml),
            Value::Fn(inner) => write!(f, "FN{}:{}", self.params, inner),
            Value::N(inner) => write!(f, "N{}:{}", self.params, inner),
            Value::Nickname(inner) => write!(f, "NICKNAME{}:{}", self.params, inner),
            Value::Photo(inner) => write!(f, "VALUE{}:{}", self.params, inner),
            Value::Birthday(inner) => write!(f, "BDAY{}:{}", self.params, inner),
            Value::Anniversary(inner) => write!(f, "ANNIVERSARY{}:{}", self.params, inner),
            Value::Gender(inner) => write!(f, "GENDER{}:{}", self.params, inner),
            Value::Adr(inner) => write!(f, "ADR{}:{}", self.params, inner),
            Value::Tel(inner) => write!(f, "TEL{}:{}", self.params, inner),
            Value::Email(inner) => write!(f, "EMAIL{}:{}", self.params, inner),
            Value::Impp(inner) => write!(f, "IMPP{}:{}", self.params, inner),
            Value::Lang(inner) => write!(f, "Lang{}:{}", self.params, inner),
            Value::Timezone(inner) => write!(f, "TZ{}:{}", self.params, inner),
            Value::Geo(inner) => write!(f, "GEO{}:{}", self.params, inner),
            Value::Title(inner) => write!(f, "TITLE{}:{}", self.params, inner),
            Value::Role(inner) => write!(f, "ROLE{}:{}", self.params, inner),
            Value::Logo(inner) => write!(f, "LOGO{}:{}", self.params, inner),
            Value::Organization(inner) => write!(f, "ORG{}:{}", self.params, inner),
            Value::Member(inner) => write!(f, "MEMBER{}:{}", self.params, inner),
            Value::Related(inner) => write!(f, "RELATED{}:{}", self.params, inner),
            Value::Categories(inner) => write!(f, "CATEGORIES{}:{}", self.params, inner),
            Value::Note(inner) => write!(f, "NOTE{}:{}", self.params, inner),
            Value::ProdID(inner) => write!(f, "PRODID{}:{}", self.params, inner),
            Value::Rev(inner) => write!(f, "REV{}:{}", self.params, inner),
            Value::Sound(inner) => write!(f, "SOUND{}:{}", self.params, inner),
            Value::Uid(inner) => write!(f, "UID{}:{}", self.params, inner),
            Value::ClientPIDMap(inner) => write!(f, "CLIENTPIDMAP{}:{}", self.params, inner),
            Value::Url(inner) => write!(f, "URL{}:{}", self.params, inner),
            Value::Version(inner) => write!(f, "VERSION{}:{}", self.params, inner),
            Value::Key(inner) => write!(f, "KEY{}:{}", self.params, inner),
            Value::FbUrl(inner) => write!(f, "FBURL{}:{}", self.params, inner),
            Value::CalAdrURL(inner) => write!(f, "CALADRURL{}:{}", self.params, inner),
            Value::CalURL(inner) => write!(f, "CALURL{}:{}", self.params, inner),
            Value::Other(name, value) => write!(f, "{}{}:{}", name, self.params, value),
        }
    }
}

impl<'a> crate::Parse<'a> for Property<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self, crate::ParseError> {
        match parse_property(input) {
            Ok(item) => Ok(item),
            Err(nom::Err::Failure(err)) => Err(nom::Err::Failure(crate::ParseError(err))),
            Err(nom::Err::Error(err)) => Err(nom::Err::Error(crate::ParseError(err))),
            Err(nom::Err::Incomplete(item)) => Err(nom::Err::Incomplete(item)),
        }
    }
}
