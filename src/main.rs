use clap::Parser;
use dialoguer::MultiSelect;

#[derive(Parser)]
struct Cli {
    #[arg(group = "input")]
    query: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Your search query: {:?}", cli.query);
    let items = vec!["Bing", "DuckDuckGo", "Google", "Yahoo!"];
    let search: Vec<usize> = MultiSelect::new().items(&items).interact().unwrap();
    println!("{:?}", search);
}
