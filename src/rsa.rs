use num::BigUint;

fn encrypt_rsa(plaintext: BigUint, modulus_n: BigUint, public_exponent: BigUint) -> BigUint {
    plaintext.modpow(&public_exponent, &modulus_n)
}

fn decrypt_rsa(ciphertext: BigUint, modulus_n: BigUint, private_exponent: BigUint) -> BigUint {
    ciphertext.modpow(&private_exponent, &modulus_n)
}

#[cfg(test)]
mod tests {
    use crate::rsa::{decrypt_rsa, encrypt_rsa};
    use num::BigUint;

    #[test]
    fn simple_wikipedia_example_encrypt() {
        assert_eq!(
            encrypt_rsa(
                BigUint::new(vec![65]),
                BigUint::new(vec![3233]),
                BigUint::new(vec![17])
            ),
            BigUint::new(vec![2790])
        );
    }

    #[test]
    fn big_numbers_encrypt() {
        assert_eq!(
            encrypt_rsa(
                BigUint::parse_bytes(b"78945612384956189456198456184651326513", 10).unwrap(),
                BigUint::parse_bytes(b"4984616654896514561389465189465156149865148956138946515613274689456148956189456195612",10).unwrap(),
                BigUint::parse_bytes(b"489156498561984651386512386512384651389465184651",10).unwrap()
            ),
            BigUint::parse_bytes(b"4734352551854118318118814748170063612738185146284502136740012597973170634016373442609",10).unwrap()
        );
    }

    #[test]
    fn simple_wikipedia_example_decrypt() {
        assert_eq!(
            decrypt_rsa(
                BigUint::new(vec![2790]),
                BigUint::new(vec![3233]),
                BigUint::new(vec![413])
            ),
            BigUint::new(vec![65])
        );
    }

    #[test]
    fn big_numbers_decrypt() {
        assert_eq!(
            decrypt_rsa(
                BigUint::parse_bytes(b"78945612384956189456198456184651326513", 10).unwrap(),
                BigUint::parse_bytes(b"4984616654896514561389465189465156149865148956138946515613274689456148956189456195612",10).unwrap(),
                BigUint::parse_bytes(b"489156498561984651386512386512384651389465184651",10).unwrap()
            ),
            BigUint::parse_bytes(b"4734352551854118318118814748170063612738185146284502136740012597973170634016373442609",10).unwrap()
        );
    }
}
