use std::fmt::{Debug, Display, Formatter, Result};

use crate::PropertyValue;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AtLeastOne<T>
where
    T: PropertyValue,
{
    Single(T),
    Multiple(Vec<T>),
}

impl<T> AtLeastOne<T>
where
    T: PropertyValue,
{
    #[allow(dead_code)]
    fn into_iterator(self) -> std::vec::IntoIter<T> {
        match self {
            AtLeastOne::Single(item) => vec![item].into_iter(),
            AtLeastOne::Multiple(multi) => multi.into_iter(),
        }
    }
}

impl<'a, T> Display for AtLeastOne<T>
where
    T: PropertyValue,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AtLeastOne::Single(raw) => write!(f, "{}", raw),
            AtLeastOne::Multiple(ty) => write!(
                f,
                "{}",
                ty.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}
