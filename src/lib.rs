pub mod analysis;
pub mod language;
pub mod launcher;
pub mod symmetric;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn load_challenges() -> HashMap<(usize, usize), Challenge> {
    let mut challenges: HashMap<(usize, usize), Challenge> = HashMap::new();
    challenges.insert(
        (1, 1),
        Challenge {
            set: 1,
            id: 1,
            title: String::from("Convert hex to base64"),
            func: hex_to_base64,
        },
    );

    challenges.insert(
        (1, 2),
        Challenge {
            set: 1,
            id: 2,
            title: String::from("Fixed XOR"),
            func: fixed_xor,
        },
    );

    challenges.insert(
        (1, 3),
        Challenge {
            set: 1,
            id: 3,
            title: String::from("Single-byte XOR cipher"),
            func: single_byte_xor,
        },
    );

    challenges.insert(
        (1, 4),
        Challenge {
            set: 1,
            id: 4,
            title: String::from("Detect single-character XOR"),
            func: detect_single_byte_xor,
        },
    );

    challenges
}

pub fn run(cli: &launcher::cli::CLI) {
    let challenges = load_challenges();

    let id = (cli.set, cli.challenge);
    let challenge = challenges.get(&id);

    match challenge {
        Some(challenge) => challenge.run(),
        None => eprintln!("No such challenge or set"),
    }
}

// 1 - 1
fn hex_to_base64() {
    let bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").expect("Invalid input");
    let b64 = base64::encode(bytes);

    println!("Encoded to: {}", b64);
    assert_eq!(
        b64,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}

// 1 - 2
fn fixed_xor() {
    let a = hex::decode("1c0111001f010100061a024b53535009181c").expect("Invalid input");
    let b = hex::decode("686974207468652062756c6c277320657965").expect("Invalid input");

    assert_eq!(
        hex::encode(symmetric::xor(&a, &b).expect("xor() returned error")),
        "746865206b696420646f6e277420706c6179",
    )
}

// 1 - 3
fn single_byte_xor() {
    let ctxt = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
        .expect("Invalid input");

    let (key, msg, distance) = analysis::single_byte_xor(&ctxt);
    // Assuming it to be UTF-8 encoded
    let msg = String::from_utf8_lossy(&msg);
    println!(
        "Broke single-byte XOR. Most likely key: {} (distance = {})\nMessage: {}",
        key, distance, msg
    );
}

fn detect_single_byte_xor() {
    let lines =
        io::BufReader::new(File::open("data/1_4.txt").expect("Error reading file 1_4.txt")).lines();

    let mut best_key: u8 = 0;
    let mut best_msg: Vec<u8> = vec![0];
    let mut lowest_distance: f64 = 1.0;

    for line in lines {
        if let Ok(line) = line {
            let ctxt = hex::decode(line).expect("Input line was not hex-encoded");
            let (key, msg, distance) = analysis::single_byte_xor(&ctxt);

            if distance < lowest_distance {
                best_key = key;
                best_msg = msg;
                lowest_distance = distance;
            }
        }
    }

    // Assuming it to be UTF-8 encoded
    let best_msg = String::from_utf8_lossy(&best_msg);
    println!(
        "Found most-likely single-byte XOR cipher. Key: {} (distance = {})\nMessage: {}",
        best_key, lowest_distance, best_msg
    );
}

struct Challenge {
    set: u32,
    id: u32,
    title: String,
    func: fn() -> (),
}

impl Challenge {
    fn title(&self) -> String {
        format!("=== {}-{}: {} ===", self.set, self.id, self.title)
    }

    fn run(&self) {
        println!("{}", self.title());
        (self.func)()
    }
}
