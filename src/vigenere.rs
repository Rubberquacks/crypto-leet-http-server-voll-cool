use crate::caesar::encrypt_caesar_char;
use crate::error::Error;

pub fn encrypt_vigenere(plaintext: &str, key: &str) -> Result<String, Error> {
    let possibly_invalid_character = key
        .chars()
        .find(|character| !('A'..='Z').contains(character));
    if let Some(character) = possibly_invalid_character {
        return Err(Error::InvalidKeyCharacterError { character });
    };

    if key.is_empty() {
        return Err(Error::EmptyKeyError);
    };

    let key_as_caesar_rotations = key.chars().map(|character| (character as u8) - (b'A'));

    plaintext
        .chars()
        .zip(key_as_caesar_rotations.cycle())
        .map(|(plaintext_character, key_caesar_rotation)| {
            encrypt_caesar_char(plaintext_character, key_caesar_rotation)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::vigenere::encrypt_vigenere;

    #[test]
    fn empty_key_returns_error() {
        match encrypt_vigenere("some_plaintext", "") {
            Err(Error::EmptyKeyError) => {}
            other => panic!("Expected empty key error but got {:#?}", other),
        }
    }

    #[test]
    fn invalid_key_character_returns_error() {
        match encrypt_vigenere("some_plaintext", "lowercase") {
            Err(Error::InvalidKeyCharacterError { character }) => {
                assert_eq!(character, 'l');
            }
            other => panic!("Expected invalid key character error but got {:#?}", other),
        }
    }

    #[test]
    fn encrypts_sample_text() {
        assert_eq!(
            encrypt_vigenere("HalloSicherheit", "ABC").unwrap(),
            String::from("HbnlpUidjesjejv")
        );
    }

    #[test]
    fn key_a_encrypts_to_same() {
        assert_eq!(
            encrypt_vigenere("HalloSicherheit", "AAA").unwrap(),
            String::from("HalloSicherheit")
        );
    }

    #[test]
    fn key_longer_than_plaintext_encrypts_correctly() {
        assert_eq!(
            encrypt_vigenere("Hi", "ABCDEFGH").unwrap(),
            String::from("Hj")
        );
    }
}
