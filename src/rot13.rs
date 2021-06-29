use crate::caesar::encrypt_caesar;
use crate::error::Error;

pub fn encrypt_rot13(to_be_rotated: &str) -> Result<String, Error> {
    encrypt_caesar(to_be_rotated, 13)
}

#[cfg(test)]
mod tests {
    use crate::rot13::encrypt_rot13;

    #[test]
    pub fn encrypt_hello() {
        assert_eq!(encrypt_rot13("Hello").unwrap(), String::from("Uryyb"))
    }
}
