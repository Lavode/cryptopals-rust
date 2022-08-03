use std::collections::HashMap;

pub mod cli;

fn load_challenges() -> HashMap<(u32, u32), Challenge> {
    let mut challenges: HashMap<(u32, u32), Challenge> = HashMap::new();
    challenges.insert(
        (1, 1),
        Challenge {
            set: 1,
            id: 1,
            title: String::from("Convert hex to base64"),
            func: hex_to_base64,
        },
    );

    challenges
}

pub fn run(opts: cli::Options) {
    let challenges = load_challenges();

    let id: (u32, u32) = (opts.set(), opts.challenge());
    let challenge = challenges.get(&id);

    match challenge {
        Some(challenge) => challenge.run(),
        None => eprintln!("No such challenge or set"),
    }
}

fn hex_to_base64() {
    let bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").expect("Invalid input");
    let b64 = base64::encode(bytes);

    println!("Encoded to: {}", b64);
    assert_eq!(
        b64,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
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
