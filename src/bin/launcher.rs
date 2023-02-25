use cryptopals::launcher::cli;

fn main() {
    let cli = cli::parse();

    cryptopals::run(&cli);
}
