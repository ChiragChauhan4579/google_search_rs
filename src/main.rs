use google_search_rs::search;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Example search query
    let search_query = "Python Programming Language";
    let max_pages = 10;
    // Call the scraping function with the search query and an optional save path
    // let df = search(search_query,max_pages,None)?;
    let df = search(search_query,max_pages,Some("results.csv"))?;
    println!("{:?}",df);
    Ok(())
}
