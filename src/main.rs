#[derive(Debug)]
struct Error {
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn main() {
    println!("{}", rot13("Hallo").unwrap());
}

fn rot13(to_be_rotated: &str) -> Result<String, Error> {
    for char in to_be_rotated.chars() {
        if !('a'..='z').contains(&char.to_ascii_lowercase()) {
            return Err(Error {
                message: String::from("Du musst zwischen a und z machen Brudi"),
            });
        }
    }

    Ok(to_be_rotated
        .chars()
        .into_iter()
        .map(|char| rot13_char(char))
        .collect())
}

fn rot13_char(char: char) -> char {
    let offset: u8;
    if ('a'..'z').contains(&char) {
        offset = 'a' as u8;
    } else if ('A'..'Z').contains(&char) {
        offset = 'A' as u8;
    } else {
        panic!("Ihr habt gemacht Bug lol.")
    }

    let char_as_byte = char as u8;

    ((((char_as_byte - offset) + 13) % 26) + offset) as char
}
