use crate::error::Error;

pub fn encrypt_caesar(to_be_rotated: &str, rotation: u8) -> Result<String, Error> {
    if !(0..=25).contains(&rotation) {
        return Err(Error::InvalidCaesarRotation { rotation });
    };

    to_be_rotated
        .chars()
        .map(|character| encrypt_caesar_char(character, rotation))
        .collect()
}

pub fn encrypt_caesar_char(character: char, rotation: u8) -> Result<char, Error> {
    let offset = if ('a'..='z').contains(&character) {
        b'a'
    } else if ('A'..='Z').contains(&character) {
        b'A'
    } else {
        return Err(Error::InvalidPlaintextCharacterError { character });
    };

    let char_as_byte = character as u8;

    Ok(((((char_as_byte - offset) + rotation) % 26) + offset) as char)
}

#[cfg(test)]
mod tests {
    use crate::caesar::encrypt_caesar;
    use crate::error::Error;

    #[test]
    fn successfully_rotate_by_1() {
        assert_eq!(encrypt_caesar("hello", 1).unwrap(), String::from("ifmmp"));
    }

    #[test]
    fn empty_string_returns_empty_string() {
        assert_eq!(encrypt_caesar("", 1).unwrap(), String::new());
    }

    #[test]
    fn rotation_of_0_does_not_change_string() {
        let to_be_rotated = String::from("Swag");
        assert_eq!(encrypt_caesar(&to_be_rotated, 0).unwrap(), to_be_rotated);
    }

    #[test]
    fn wrapping_around_z_works() {
        assert_eq!(encrypt_caesar("z", 1).unwrap(), String::from("a"));
        assert_eq!(encrypt_caesar("Z", 1).unwrap(), String::from("A"));
    }

    #[test]
    fn preserves_case() {
        assert_eq!(encrypt_caesar("AaBbCc", 3).unwrap(), String::from("DdEeFf"));
    }

    #[test]
    fn ascii_character_that_is_not_a_letter() {
        match encrypt_caesar("AaBb.Cc", 3) {
            Err(Error::InvalidPlaintextCharacterError { character }) => assert_eq!(character, '.'),
            other => panic!("{:#?} was not the expected error.", other),
        }
    }

    #[test]
    fn non_ascii_character() {
        match encrypt_caesar("AaBbλCc", 3) {
            Err(Error::InvalidPlaintextCharacterError { character }) => assert_eq!(character, 'λ'),
            other => panic!("{:#?} was not the expected error.", other),
        }
    }

    #[test]
    fn rotation_over_25() {
        match encrypt_caesar("some_random_string", 26) {
            Err(Error::InvalidCaesarRotation { rotation }) => assert_eq!(rotation, 26),
            other => panic!("{:#?} was not the expected error.", other),
        }
    }
}
