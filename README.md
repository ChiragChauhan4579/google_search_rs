# google_search_rs

google_search_rs allows you to scrape Google search results programmatically using a headless browser, extract relevant information (titles, URLs, and snippets), and store the results in a polars::DataFrame or CSV file.

## Features
Headless Browsing: Automates browser interactions using headless_chrome.
Google Search Scraping: Extracts titles, URLs, and snippets from search results.
Multiple Pages: Scrapes multiple pages of search results, if available.
Data Storage: Returns the scraped data as a Polars DataFrame and optionally saves it as a CSV file.

## To install add in Cargo.toml
`google_search_rs = "0.1.0"`

## Usage

```rust
use google_search_rs::search;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let search_query = "Rust Programming Language";
    let max_pages = 10;

    // To only retrieve a dataframe
    // let df = search(search_query,max_pages,None)?;

    // To save the results in a csv
    let df = search(search_query,max_pages,Some("results.csv"))?; // This will scrape the first ten pages of Google search results for "Rust Programming Language" and save the results in a CSV file called results.csv.


    println!("{:?}",df);
    Ok(())
}
```

## Output
The extracted search results will contain the following columns:
Title: The title of the search result.
URL: The URL of the search result.
Snippet: A brief description or snippet from the search result.

## License
This project is licensed under the MIT License.

