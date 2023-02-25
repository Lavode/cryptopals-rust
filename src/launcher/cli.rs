use clap::Parser;

pub fn parse() -> CLI {
    CLI::parse()
}

#[derive(Parser, Debug)]
pub struct CLI {
    #[arg(long, help = "Number of challenge set")]
    pub set: usize,

    #[arg(long, help = "Number of challenge")]
    pub challenge: usize,
}
