use std::borrow::Cow;
use std::fmt::{Debug, Display};

mod ty;
pub use self::ty::TypeOrRaw;

mod at_least;
pub use self::at_least::AtLeastOne;

use crate::types::{Address, Email, FormattedName, Gender, GeoPosition, Image, Kind, Url};

#[cfg(feature = "serde")]
pub trait PropertyValue: Debug + PartialEq + Clone + Display + serde::Serialize {}
#[cfg(not(feature = "serde"))]
pub trait PropertyValue: Debug + PartialEq + Clone + Display {}

#[cfg(not(feature = "serde"))]
impl<T> PropertyValue for T where T: Debug + PartialEq + Clone + Display {}
#[cfg(feature = "serde")]
impl<T> PropertyValue for T where T: Debug + PartialEq + Clone + Display + serde::Serialize {}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value<'a> {
    Begin,
    End,
    Source(TypeOrRaw<Url<'a>>),
    Kind(TypeOrRaw<Kind>),
    Xml(Cow<'a, str>),
    Fn(Cow<'a, str>),
    N(TypeOrRaw<FormattedName<'a>>),
    Nickname(AtLeastOne<Cow<'a, str>>),
    Photo(TypeOrRaw<Image<'a>>),
    Birthday(TypeOrRaw<chrono::NaiveDate>),
    Anniversary(TypeOrRaw<chrono::NaiveDate>),
    Gender(TypeOrRaw<Gender<'a>>),
    Adr(TypeOrRaw<Address<'a>>),
    Tel(Cow<'a, str>),
    Email(TypeOrRaw<Email<'a>>),
    Impp(Cow<'a, str>),
    Lang(Cow<'a, str>),
    Timezone(Cow<'a, str>),
    Geo(TypeOrRaw<GeoPosition>),
    Title(Cow<'a, str>),
    Role(Cow<'a, str>),
    Logo(TypeOrRaw<Image<'a>>),
    Organization(Cow<'a, str>),
    Member(Cow<'a, str>),
    Related(Cow<'a, str>),
    Categories(AtLeastOne<Cow<'a, str>>),
    Note(Cow<'a, str>),
    ProdID(Cow<'a, str>),
    Rev(Cow<'a, str>),
    Sound(Cow<'a, str>),
    Uid(Cow<'a, str>),
    ClientPIDMap(Cow<'a, str>),
    Url(TypeOrRaw<Url<'a>>),
    Version(Cow<'a, str>),
    Key(Cow<'a, str>),
    FbUrl(Cow<'a, str>),
    CalAdrURL(Cow<'a, str>),
    CalURL(Cow<'a, str>),
    Other(Cow<'a, str>, Cow<'a, str>),
}

impl<'a> Value<'a> {
    pub fn name_raw(&self) -> &str {
        match &self {
            Value::Begin => "BEGIN",
            Value::End => "END",
            Value::Source(_) => "SOURCE",
            Value::Kind(_) => "KIND",
            Value::Xml(_) => "XML",
            Value::Fn(_) => "FN",
            Value::N(_) => "N",
            Value::Nickname(_) => "NICKNAME",
            Value::Photo(_) => "PHOTO",
            Value::Birthday(_) => "BIRTHDAY",
            Value::Anniversary(_) => "ANNIVERSARY",
            Value::Gender(_) => "GENDER",
            Value::Adr(_) => "ADR",
            Value::Tel(_) => "PHONE",
            Value::Email(_) => "EMAIL",
            Value::Impp(_) => "IMPP",
            Value::Lang(_) => "LANG",
            Value::Timezone(_) => "TZ",
            Value::Geo(_) => "GEO",
            Value::Title(_) => "TITLE",
            Value::Role(_) => "ROLE",
            Value::Logo(_) => "LOGO",
            Value::Organization(_) => "ORGANIZATION",
            Value::Member(_) => "MEMBER",
            Value::Related(_) => "RELATED",
            Value::Categories(_) => "CATEGORIES",
            Value::Note(_) => "NOTE",
            Value::ProdID(_) => "PRODID",
            Value::Rev(_) => "REV",
            Value::Sound(_) => "SOUND",
            Value::Uid(_) => "UID",
            Value::ClientPIDMap(_) => "CLIENTPIDMAP",
            Value::Url(_) => "URL",
            Value::Version(_) => "VERSION",
            Value::Key(_) => "Key",
            Value::FbUrl(_) => "FBURL",
            Value::CalAdrURL(_) => "CALADRURL",
            Value::CalURL(_) => "CALURL",
            Value::Other(name, _) => name,
        }
    }

    pub fn name_label(&self) -> &str {
        match self {
            Value::Begin => "Begin",
            Value::End => "End",
            Value::Source(_) => "Source",
            Value::Kind(_) => "Kind",
            Value::Xml(_) => "Xml",
            Value::Fn(_) => "Formatted Name",
            Value::N(_) => "Name",
            Value::Nickname(_) => "Nickname",
            Value::Photo(_) => "Photo",
            Value::Birthday(_) => "Birthday",
            Value::Anniversary(_) => "Anniversary",
            Value::Gender(_) => "Gender",
            Value::Adr(_) => "Address",
            Value::Tel(_) => "Phone",
            Value::Email(_) => "Email",
            Value::Impp(_) => "Impp",
            Value::Lang(_) => "Language",
            Value::Timezone(_) => "Timezone",
            Value::Geo(_) => "Location",
            Value::Title(_) => "Title",
            Value::Role(_) => "Role",
            Value::Logo(_) => "Logo",
            Value::Organization(_) => "Organization",
            Value::Member(_) => "Member",
            Value::Related(_) => "Related",
            Value::Categories(_) => "Categories",
            Value::Note(_) => "Note",
            Value::ProdID(_) => "Production Id",
            Value::Rev(_) => "Revision",
            Value::Sound(_) => "Sound",
            Value::Uid(_) => "Uid",
            Value::ClientPIDMap(_) => "Client PID Map",
            Value::Url(_) => "Url",
            Value::Version(_) => "Version",
            Value::Key(_) => "Key",
            Value::FbUrl(_) => "FBURL",
            Value::CalAdrURL(_) => "Calendar Address URL",
            Value::CalURL(_) => "Calendar URL",
            Value::Other(name, _) => name,
        }
    }
}
