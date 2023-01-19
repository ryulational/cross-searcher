use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(group = "input")]
    query: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Your search query: {:?}", cli.query);
}
