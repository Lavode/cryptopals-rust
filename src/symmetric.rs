// xor calculates the byte-wise XOR of two byte vectors of equal length.
//
// An error is returned if the two inputs differ in length.
pub fn xor(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, String> {
    if a.len() == b.len() {
        Ok(a.iter().zip(b).map(|(x, y)| x ^ y).collect())
    } else {
        Err(String::from(format!(
            "Input vectors must be of equal length, but a was {} and b {} bytes",
            a.len(),
            b.len()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_equal_length() {
        let a: Vec<u8> = vec![0x00, 0x27, 0x5A, 0x47];
        let b: Vec<u8> = vec![0xFF, 0x67, 0xDE, 0x9B];

        assert_eq!(
            xor(a, b).expect("xor() returned error"),
            vec![0xFF, 0x40, 0x84, 0xDC]
        );
    }

    #[test]
    fn xor_differing_length() {
        let a: Vec<u8> = vec![0x00, 0x01, 0x02];
        let b: Vec<u8> = vec![0x00, 0x01];

        assert!(xor(a, b).is_err());
    }
}
