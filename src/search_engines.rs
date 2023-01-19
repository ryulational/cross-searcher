use crate::search_engines;

#[derive(Clone, Debug)]
pub struct SearchEngine {
    pub name: String,
    pub pattern: String,
    pub divider: String,
}

pub fn get_search_engines() -> Vec<SearchEngine> {
    vec![get_bing(), get_duckduckgo(), get_google(), get_yahoo()]
}

fn get_bing() -> SearchEngine {
    SearchEngine {
        name: String::from("Bing"),
        pattern: String::from("https://www.bing.com/search?q="),
        divider: String::from(" "),
    }
}

fn get_duckduckgo() -> SearchEngine {
    SearchEngine {
        name: String::from("DuckDuckgo"),
        pattern: String::from("https://duckduckgo.com/?q="),
        divider: String::from("+"),
    }
}

fn get_google() -> SearchEngine {
    SearchEngine {
        name: String::from("Google"),
        pattern: String::from("https://www.google.com/search?q="),
        divider: String::from(" "),
    }
}

fn get_yahoo() -> SearchEngine {
    SearchEngine {
        name: String::from("Yahoo!"),
        pattern: String::from("https://search.yahoo.com/search?p="),
        divider: String::from(" "),
    }
}
