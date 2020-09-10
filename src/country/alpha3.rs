use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum Alpha3 {
    None,
    // ENUM START
    // ENUM END
}

impl Alpha3 {
    pub fn is_none(&self) -> bool {
        match *self {
            Alpha3::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Alpha3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
