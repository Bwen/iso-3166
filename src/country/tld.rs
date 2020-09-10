use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum Tld {
    None,
    // ENUM START
    // ENUM END
}

impl Tld {
    pub fn is_none(&self) -> bool {
        match *self {
            Tld::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Tld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{:?}", self)
    }
}
