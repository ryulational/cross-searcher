mod search_engines;

use clap::Parser;
use dialoguer::MultiSelect;
use webbrowser;

#[derive(Parser)]
struct Cli {
    #[arg(group = "input")]
    query: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Your search query: {}", cli.query);

    let engines: Vec<search_engines::SearchEngine> = search_engines::get_search_engines();

    let engine_names: Vec<String> = engines.clone().into_iter().map(|x| x.name).collect();
    let selections: Vec<usize> = MultiSelect::new().items(&engine_names).interact().unwrap();
    let selected_engines: Vec<&search_engines::SearchEngine> =
        selections.into_iter().map(|i| &engines[i]).collect();
    println!("{:?}", selected_engines);
    let search_terms: Vec<&str> = cli.query.split(" ").collect();
    let search_urls: Vec<String> = selected_engines
        .into_iter()
        .map(|e| [e.clone().pattern, search_terms.join(&e.divider)].join(""))
        .collect();

    for url in search_urls {
        if webbrowser::open(&url).is_ok() {
            println!("Browser opened: {}", &url);
        }
    }
}
