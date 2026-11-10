use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DbError {
    InvalidInteger,
    IntegerOverflow,
    IntegerUnderflow,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInteger => write!(f, "value is not a valid integer"),
            Self::IntegerOverflow => write!(f, "integer overflow"),
            Self::IntegerUnderflow => write!(f, "integer underflow"),
        }
    }
}

impl Error for DbError {}