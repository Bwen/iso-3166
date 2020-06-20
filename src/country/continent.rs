use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum ContinentCode {
    None,
    // ENUM START
    AF,
    AS,
    EU,
    NA,
    OC,
    SA,
    // ENUM END
}

impl ContinentCode {
    pub fn is_none(&self) -> bool {
        match *self {
            ContinentCode::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for ContinentCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
