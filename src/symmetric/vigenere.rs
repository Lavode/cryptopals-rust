use super::xor_repeating;

/// Vigenere cipher extended to arbitrary byte values.
pub struct Vigenere {
    key: Vec<u8>,
}

impl Vigenere {
    pub fn new(key: Vec<u8>) -> Vigenere {
        Vigenere { key }
    }

    /// Encrypt a plaintext message.
    pub fn encrypt(&self, msg: Vec<u8>) -> Vec<u8> {
        xor_repeating(&msg, &self.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let cipher = Vigenere::new("YELLOW SUBMARINE".as_bytes().to_vec());
        let msg = "Hello world, this is your captain speaking."
            .as_bytes()
            .to_vec();
        let ctxt = cipher.encrypt(msg);

        assert_eq!(
            ctxt,
            vec![
                17, 32, 32, 32, 32, 119, 87, 60, 39, 46, 41, 109, 114, 61, 38, 44, 42, 101, 37, 63,
                111, 46, 79, 38, 39, 98, 46, 32, 34, 61, 47, 44, 55, 101, 63, 60, 42, 54, 75, 58,
                59, 37, 99
            ]
        )
    }
}
