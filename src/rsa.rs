use num::{BigUint, Integer};

fn encrypt_rsa(plaintext: BigUint, modulus_n: BigUint, public_exponent: BigUint) -> BigUint {
    plaintext.modpow(&public_exponent, &modulus_n)
}

fn decrypt_rsa(ciphertext: BigUint, modulus_n: BigUint, private_exponent: BigUint) -> BigUint {
    ciphertext.modpow(&private_exponent, &modulus_n)
}

// source: https://en.wikipedia.org/wiki/Primality_test#Python_code
fn is_prime(n: &BigUint) -> bool {
    if n == &BigUint::from(2u64) || n == &BigUint::from(3u64) {
        return true;
    }

    if n <= &BigUint::from(1u64)
        || n.is_multiple_of(&BigUint::from(2u64))
        || n.is_multiple_of(&BigUint::from(3u64))
    {
        return false;
    }

    let mut i = BigUint::from(5u64);
    while &i.pow(2) <= n {
        if n.is_multiple_of(&i) || n.is_multiple_of(&(&i + &BigUint::from(2u64))) {
            return false;
        }
        i += BigUint::from(6u64);
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::rsa::{decrypt_rsa, encrypt_rsa, is_prime};
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

    #[test]
    fn simple_prime() {
        assert!(is_prime(&BigUint::from(101u64)));
        assert!(is_prime(&BigUint::from(7u64)));
        assert!(is_prime(&BigUint::from(31u64)));
        assert!(is_prime(&BigUint::from(2u64)));
        assert!(is_prime(&BigUint::from(401u64)));
        assert!(is_prime(&BigUint::from(337u64)));
    }

    #[test]
    fn simple_not_prime() {
        assert!(!is_prime(&BigUint::from(10u64)));
        assert!(!is_prime(&BigUint::from(1u64)));
        assert!(!is_prime(&BigUint::from(49u64)));
        assert!(!is_prime(&BigUint::from(121u64)));
        assert!(!is_prime(&BigUint::from(763u64)));
        assert!(!is_prime(&BigUint::from(333u64)));
        assert!(!is_prime(&BigUint::from(237u64)));
    }

    #[test]
    fn big_prime_numbers() {
        assert!(is_prime(&BigUint::parse_bytes(b"961748941", 10).unwrap()));
        let x = BigUint::parse_bytes(b"1000000000000066600000000000001", 10).unwrap();
        println!("{}", x.bits());
        assert!(is_prime(&x));
    }
}
