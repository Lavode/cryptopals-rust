/// Calculates the byte-wise XOR of two byte vectors of equal length.
///
/// An error is returned if the two inputs differ in length.
pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Result<Vec<u8>, String> {
    if a.len() == b.len() {
        Ok(xor_repeating(a, b))
    } else {
        Err(String::from(format!(
            "Input vectors must be of equal length, but a was {} and b {} bytes",
            a.len(),
            b.len()
        )))
    }
}

/// Calculates the byte-wise XOR of two byte vectors of arbitrary length.
///
/// The resulting vector has the same length as the first argument. If the second argument is
/// shorter, access to it will 'wrap around'.
pub fn xor_repeating(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let b_repeating = b.iter().cycle();
    a.iter().zip(b_repeating).map(|(x, y)| x ^ y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_equal_length() {
        let a: Vec<u8> = vec![0x00, 0x27, 0x5A, 0x47];
        let b: Vec<u8> = vec![0xFF, 0x67, 0xDE, 0x9B];

        assert_eq!(
            xor(&a, &b).expect("xor() returned error"),
            vec![0xFF, 0x40, 0x84, 0xDC]
        );
    }

    #[test]
    fn xor_differing_length() {
        let a: Vec<u8> = vec![0x00, 0x01, 0x02];
        let b: Vec<u8> = vec![0x00, 0x01];

        assert!(xor(&a, &b).is_err());
    }

    #[test]
    fn xor_repeating() {
        let a: Vec<u8> = vec![0x00, 0x27, 0x5A, 0x47, 0xB3];
        let b: Vec<u8> = vec![0xFF, 0x67];

        assert_eq!(
            super::xor_repeating(&a, &b),
            vec![0xFF, 0x40, 0xA5, 0x20, 0x4C]
        );
    }
}
