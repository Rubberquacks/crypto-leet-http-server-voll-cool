#[derive(Debug)]
pub enum Error {
    InvalidPlaintextCharacterError { character: char },
    InvalidCaesarRotation { rotation: u8 },
    InvalidKeyCharacterError { character: char },
    EmptyKeyError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Error::InvalidPlaintextCharacterError { character } => format!(
                "Found invalid character '{}'. Must be between a and z.",
                character
            ),
            Error::InvalidCaesarRotation { rotation } => {
                format!(
                    "Found invalid rotation {}. Must be between 0 and 25",
                    rotation
                )
            }
            Error::EmptyKeyError => String::from("Key cannot be empty."),
            Error::InvalidKeyCharacterError { character } => format!(
                "Found invalid key character '{}'. Must be between uppercase A-Z.",
                character
            ),
        };
        write!(f, "{}", message)
    }
}
