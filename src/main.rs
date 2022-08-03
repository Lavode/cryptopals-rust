use cryptopals::cli::Options;
use std::env;
use std::process;

fn main() {
    let opts = Options::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        usage();
    });

    cryptopals::run(opts);
}

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("./cryptopals <set> <challenge>");
    eprintln!("Example: ./cryptopals 2 4");
    process::exit(1);
}
