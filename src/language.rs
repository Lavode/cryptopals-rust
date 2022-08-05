use std::collections::HashMap;

fn frequency_analysis(text: &str) -> HashMap<char, f64> {
    let counts = count_letters(text);
    let total = counts.values().sum::<u32>() as f64;

    counts
        .into_iter()
        .map(|(k, v)| (k, (v as f64) / total))
        .collect()
}

fn count_letters(text: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for chr in text.to_lowercase().chars() {
        let freq = counts.entry(chr).or_insert(0);
        *freq += 1;
    }

    counts
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn count_letters() {
        let sentence = "Because that's how the zebra do";
        let counts = HashMap::from([
            ('a', 3),
            ('b', 2),
            ('c', 1),
            ('d', 1),
            ('e', 4),
            ('h', 3),
            ('o', 2),
            ('r', 1),
            ('s', 2),
            ('t', 3),
            ('u', 1),
            ('w', 1),
            ('z', 1),
            ('\'', 1),
            (' ', 5),
        ]);

        assert_eq!(super::count_letters(sentence), counts);
    }

    #[test]
    fn frequency_analysis() {
        let sentence = "Because that's how the zebra do";
        let freqs = HashMap::from([
            ('a', 3.0 / 31.0),
            ('b', 2.0 / 31.0),
            ('c', 1.0 / 31.0),
            ('d', 1.0 / 31.0),
            ('e', 4.0 / 31.0),
            ('h', 3.0 / 31.0),
            ('o', 2.0 / 31.0),
            ('r', 1.0 / 31.0),
            ('s', 2.0 / 31.0),
            ('t', 3.0 / 31.0),
            ('u', 1.0 / 31.0),
            ('w', 1.0 / 31.0),
            ('z', 1.0 / 31.0),
            ('\'', 1.0 / 31.0),
            (' ', 5.0 / 31.0),
        ]);

        let actual = super::frequency_analysis(sentence);
        assert_eq!(actual.len(), freqs.len());
        for (k, v) in freqs {
            assert_approx_eq!(actual.get(&k).expect("Key missing in output"), &v);
        }
    }
}
