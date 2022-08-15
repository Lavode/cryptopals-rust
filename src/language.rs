use std::collections::{HashMap, HashSet};

/// Average frequencies of letters in the English language.
///
/// Based on
/// http://www.practicalcryptography.com/cryptanalysis/letter-frequencies-various-languages/english-letter-frequencies/,
/// with the frequency of the space character added by assuming an average word length of six
/// characters.
pub fn english_frequencies() -> HashMap<char, f64> {
    let mut freqs: HashMap<char, f64> = HashMap::new();
    freqs.insert(' ', 0.16666666666666666);
    freqs.insert('a', 0.09037099672339077);
    freqs.insert('b', 0.01691153155057605);
    freqs.insert('c', 0.0334002748123877);
    freqs.insert('d', 0.04090476693795582);
    freqs.insert('e', 0.12789345735123137);
    freqs.insert('f', 0.02304196173765987);
    freqs.insert('g', 0.02209068808793996);
    freqs.insert('h', 0.05242574780678575);
    freqs.insert('i', 0.07747595391607652);
    freqs.insert('j', 0.0023253355882042067);
    freqs.insert('k', 0.008561462847479126);
    freqs.insert('l', 0.04449846739245323);
    freqs.insert('m', 0.026741359264348376);
    freqs.insert('n', 0.07578480076101891);
    freqs.insert('o', 0.02187929394355776);
    freqs.insert('p', 0.02187929394355776);
    freqs.insert('q', 0.0010569707219110032);
    freqs.insert('r', 0.0669062466969665);
    freqs.insert('s', 0.07113412958461052);
    freqs.insert('t', 0.09449318253884367);
    freqs.insert('u', 0.028326815347214884);
    freqs.insert('v', 0.011203889652256634);
    freqs.insert('w', 0.019342564210971358);
    freqs.insert('x', 0.002008244371630906);
    freqs.insert('y', 0.018179896416869252);
    freqs.insert('z', 0.0011626677941021033);

    freqs
}

/// Performs frequency analysis of the chars (unicode scalars) in a string.
///
/// As this works on unicode scalars rather than grapheme clusters, you should only expect sensible
/// results for alphabets where there is a 1:1 correspondence most of the time.
pub fn frequency_analysis(text: &str) -> HashMap<char, f64> {
    let counts = count_letters(text);
    let total = counts.values().sum::<u32>() as f64;

    counts
        .into_iter()
        .map(|(k, v)| (k, (v as f64) / total))
        .collect()
}

/// Counts the occurence of chars (unicode scalars) in a string.
fn count_letters(text: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for chr in text.to_lowercase().chars() {
        let freq = counts.entry(chr).or_insert(0);
        *freq += 1;
    }

    counts
}

/// Calculates the difference between two frequency histograms.
///
/// Calculation is based on the Hellinger histogram difference:
/// 1/sqrt(2) * sqrt[SUM{(sqrt(p_i) - sqrt(q_i))^2}]
///
/// Two equal histograms will have a difference of 0, while two completely disjoint ones will have
/// a difference of 1.
pub fn histogram_difference(a: &HashMap<char, f64>, b: &HashMap<char, f64>) -> f64 {
    // If we don't specify the type, inference of the hasher type will fail. See
    // https://stackoverflow.com/questions/62949404/cannot-infer-type-for-type-parameter-s-when-using-hashsetfrom-iter
    let keys = HashSet::<_>::from_iter(a.keys().chain(b.keys()));

    // Hellinger distance is 1/sqrt(2) * sqrt[SUM{(sqrt(p_i) - sqrt(q_i))^2}]
    let mut dist = 0.0;
    for k in keys {
        let p = a.get(k).unwrap_or(&0.0);
        let q = b.get(k).unwrap_or(&0.0);

        dist += (p.sqrt() - q.sqrt()).powi(2);
    }

    let sqrt_two = (2.0 as f64).sqrt();
    dist = dist.sqrt() / sqrt_two;

    dist
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

    #[test]
    fn histogram_difference() {
        let a: HashMap<char, f64> = HashMap::from([
            ('a', 0.3),
            ('b', 0.1),
            ('c', 0.2),
            ('d', 0.25),
            ('e', 0.05),
            ('f', 0.1),
        ]);

        let b: HashMap<char, f64> = HashMap::from([('g', 0.5), ('h', 0.5)]);

        let c: HashMap<char, f64> = HashMap::from([
            ('a', 0.15),
            ('c', 0.35),
            ('d', 0.30),
            ('f', 0.15),
            ('i', 0.05),
        ]);

        assert_approx_eq!(super::histogram_difference(&a, &a), 0.0);
        assert_approx_eq!(super::histogram_difference(&a, &b), 1.0);
        assert_approx_eq!(super::histogram_difference(&a, &c), 0.35631035439043124);
    }
}
