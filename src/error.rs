use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidIndex;

impl Display for InvalidIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid index")
    }
}

impl Error for InvalidIndex {}

#[derive(Debug)]
/// Error type when determining plane normal.
pub struct CollinearPoints;

impl Display for CollinearPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The given points are on one line!")
    }
}

impl Error for CollinearPoints {}
