//! # rolodex
//!
//! rolodex represents vcard as a vector of properties.

pub mod value;
pub use self::value::{AtLeastOne, PropertyValue, TypeOrRaw, Value};

pub mod types;

pub mod property;
pub use self::property::{Parameter, Parameters, Property};

mod vcard;
pub use self::vcard::VCard;

pub mod parse;
pub use self::parse::{Parse, ParseError};

pub const DATE_FORMAT: &str = "%Y%m%d";
