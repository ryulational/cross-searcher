use clap::Parser;
use dialoguer::MultiSelect;
use webbrowser;

#[derive(Parser)]
struct Cli {
    #[arg(group = "input")]
    query: String,
}

struct SearchEngine {
    name: String,
    pattern: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Your search query: {:?}", cli.query);

    let bing: SearchEngine = SearchEngine {
        name: String::from("Bing"),
        pattern: String::from("https://www.bing.com/search?q="),
    };

    let duckduckgo: SearchEngine = SearchEngine {
        name: String::from("DuckDuckgo"),
        pattern: String::from("https://duckduckgo.com/?q="),
    };

    let google = SearchEngine {
        name: String::from("Google"),
        pattern: String::from("https://www.google.com/search?q="),
    };

    let yahoo: SearchEngine = SearchEngine {
        name: String::from("Yahoo!"),
        pattern: String::from("https://search.yahoo.com/search?p="),
    };

    let engines: Vec<SearchEngine> = vec![bing, duckduckgo, google, yahoo];
    let items: Vec<String> = engines.into_iter().map(|x| x.name).collect();
    let search: Vec<usize> = MultiSelect::new().items(&items).interact().unwrap();
    println!("{:?}", search);

    if webbrowser::open("https://example.com").is_ok() {
        println!("Browser opened");
    }
}
