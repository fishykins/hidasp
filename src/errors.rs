use std::{error::Error as stde, fmt};

#[derive(Debug, Clone)]
pub struct Error(String);

impl stde for Error {}

impl Error {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "joystick error")
    }
}
