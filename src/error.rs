use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct InvalidIndex;

impl Display for InvalidIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid index")
    }
}

impl Error for InvalidIndex {}
