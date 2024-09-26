#![allow(unused_imports)]
#![allow(unused_variables)]
use scraper::{self, Html, Selector};
use headless_chrome::Browser;
use reqwest;
use serde_json::Value;
use std::error::Error;
use polars::prelude::*;
use urlencoding;

pub fn search(search_query: &str,max_pages: usize,save_to_path: Option<&str>) -> Result<DataFrame, Box<dyn Error>> {
    // Step 1: Launch the headless browser
    let browser = Browser::default().unwrap();

    // Step 2: Open a new tab and navigate to the Google search page
    let tab = browser.new_tab()?;
    let search_url = format!("https://www.google.com/search?q={}", urlencoding::encode(search_query));
    tab.navigate_to(&search_url)?;

    let result_selector = Selector::parse("div.g").unwrap();
    let title_selector = Selector::parse("h3").unwrap();
    let url_selector = Selector::parse("a").unwrap();
    let snippet_selector = Selector::parse("div.VwiC3b").unwrap();

    // Create vectors to store data
    let mut titles = Vec::new();
    let mut urls = Vec::new();
    let mut snippets = Vec::new();

    // Step 3: Loop through multiple pages
    for page in 1..=max_pages {
        // Wait for the search results to load
        tab.wait_for_element("div.g")?;

        // Step 4: Get the page's inner HTML
        let page_html_value = tab.evaluate("document.documentElement.outerHTML", false)?.value.unwrap();
        let page_html = match page_html_value {
            Value::String(html) => html,
            _ => return Err("Failed to extract HTML content as a string.".into()),
        };

        // Step 5: Parse the HTML with scraper
        let document = Html::parse_document(&page_html);

        // Step 6: Extract the search results
        for result in document.select(&result_selector) {
            // Extract title
            let title = result.select(&title_selector)
                .next()
                .map(|element| element.text().collect::<Vec<_>>().join(" "))
                .unwrap_or_default();

            // Extract URL
            let url = result.select(&url_selector)
                .next()
                .and_then(|element| element.value().attr("href"))
                .map(|s| s.to_string())
                .unwrap_or_default();

            // Extract snippet
            let snippet = result.select(&snippet_selector)
                .next()
                .map(|element| element.text().collect::<Vec<_>>().join(" "))
                .unwrap_or_default();

            // Store the extracted information
            titles.push(title);
            urls.push(url);
            snippets.push(snippet);
        }

        // Step 7: Navigate to the next page if max_pages > 1
        if page < max_pages {
            // Find and click the "Next" button, or end the loop if not found
            if let Ok(next_button) = tab.wait_for_element("a#pnnext") {
                next_button.click()?;
            } else {
                println!("No more pages to scrape.");
                break;
            }
        }
    }

    // Step 8: Create a DataFrame from the vectors
    let df = DataFrame::new(vec![
        Series::new("Title".into(), titles),
        Series::new("URL".into(), urls),
        Series::new("Snippet".into(), snippets),
    ])?;

    // Step 9: Save the DataFrame if a path is provided
    if let Some(path) = save_to_path {
        let file = std::fs::File::create(path)?;
        let mut writer = CsvWriter::new(file);
        writer.finish(&mut df.clone())?;
        println!("DataFrame saved to: {}", path);
    } else {
        println!("No path provided, DataFrame not saved.");
    }

    Ok(df)
}
