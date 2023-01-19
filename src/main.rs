use clap::Parser;
use dialoguer::MultiSelect;
use webbrowser;

#[derive(Parser)]
struct Cli {
    #[arg(group = "input")]
    query: String,
}

#[derive(Clone, Debug)]
struct SearchEngine {
    name: String,
    pattern: String,
    divider: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Your search query: {:?}", cli.query);

    let bing: SearchEngine = SearchEngine {
        name: String::from("Bing"),
        pattern: String::from("https://www.bing.com/search?q="),
        divider: String::from(" "),
    };

    let duckduckgo: SearchEngine = SearchEngine {
        name: String::from("DuckDuckgo"),
        pattern: String::from("https://duckduckgo.com/?q="),
        divider: String::from("+"),
    };

    let google = SearchEngine {
        name: String::from("Google"),
        pattern: String::from("https://www.google.com/search?q="),
        divider: String::from(" "),
    };

    let yahoo: SearchEngine = SearchEngine {
        name: String::from("Yahoo!"),
        pattern: String::from("https://search.yahoo.com/search?p="),
        divider: String::from(" "),
    };

    let engines: Vec<SearchEngine> = vec![bing, duckduckgo, google, yahoo];
    let items: Vec<String> = engines.clone().into_iter().map(|x| x.name).collect();
    let selections: Vec<usize> = MultiSelect::new().items(&items).interact().unwrap();
    let selected_engines: Vec<&SearchEngine> =
        selections.into_iter().map(|i| &engines[i]).collect();
    println!("{:?}", selected_engines);
    let search_terms: Vec<&str> = cli.query.split(" ").collect();
    let search_urls: Vec<String> = selected_engines
        .into_iter()
        .map(|e| [e.clone().pattern, search_terms.join(&e.divider)].join(""))
        .collect();
    println!("{:?}", search_urls);

    if webbrowser::open("https://example.com").is_ok() {
        println!("Browser opened");
    }
}
