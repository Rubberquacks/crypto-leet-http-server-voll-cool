#[derive(Debug)]
pub enum Error {
    InvalidCharacterError { character: char },
    InvalidCaesarRotation { rotation: u8 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Error::InvalidCharacterError { character } => format!(
                "Found invalid character {}. Must be between a and z.",
                character
            ),
            Error::InvalidCaesarRotation { rotation } => {
                format!(
                    "Found invalid rotation {}. Must be between 0 and 25",
                    rotation
                )
            }
        };
        write!(f, "{}", message)
    }
}
