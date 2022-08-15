use crate::{language, symmetric};

pub fn single_byte_xor(ctxt: &Vec<u8>) -> (u8, Vec<u8>, f64) {
    let mut best_key = 0;
    let mut lowest_distance = 1.0;

    for key in 0u8..255 {
        let msg = symmetric::xor_repeating(ctxt, &vec![key]);
        // Our potential message is assumed to be encoded as UTF-8
        let msg = String::from_utf8_lossy(&msg);

        let freqs = language::frequency_analysis(&msg);
        let distance = language::histogram_difference(&freqs, &language::english_frequencies());
        if distance < lowest_distance {
            best_key = key;
            lowest_distance = distance;
        }
    }

    (
        best_key,
        symmetric::xor_repeating(ctxt, &vec![best_key]),
        lowest_distance,
    )
}
