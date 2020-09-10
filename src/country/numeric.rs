use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum Numeric {
    None,
    // ENUM START
    // ENUM END
}

impl Numeric {
    pub fn is_none(&self) -> bool {
        match *self {
            Numeric::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
