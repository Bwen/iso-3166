use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum Alpha2 {
    None,
    // ENUM START
    // ENUM END
}

impl Alpha2 {
    pub fn is_none(&self) -> bool {
        match *self {
            Alpha2::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Alpha2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
