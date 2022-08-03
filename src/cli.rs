pub struct Options {
    set: u32,
    challenge: u32,
}

impl Options {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Options, &'static str> {
        // Skip first argument, which is program name
        args.next();

        let set = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing argument <set>"),
        };

        let challenge = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing argument <challenge>"),
        };

        let set: u32 = match set.parse() {
            Ok(v) => v,
            Err(_) => return Err("Invalid value for set, must be numerical"),
        };

        let challenge: u32 = match challenge.parse() {
            Ok(v) => v,
            Err(_) => return Err("Invalid value for challenge, must be numerical"),
        };

        Ok(Options { set, challenge })
    }

    pub fn set(&self) -> u32 {
        self.set
    }

    pub fn challenge(&self) -> u32 {
        self.challenge
    }
}
