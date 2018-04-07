use std::env;
use std::fs::File;
use std::io::Write;

extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Name, Predicate};

fn main() {
    let args: Vec<String> = env::args().collect();
    let playlist_url = &args[1];
    get_links(playlist_url);
}

fn get_links(url: &str) {
    let mut links_file = File::create("links_file.txt").expect("Error on creating file");

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    for node in document.find(Name("tr")) {
        let url = node.find(Name("td").descendant(Name("a")))
            .next()
            .unwrap();
        let url_txt = format!("https://www.radiojavan.com{} \n", url.attr("href").unwrap());
        links_file.write_all(url_txt.as_bytes()).expect("Error on writing file");
    }
    println!("All Done!")
}

