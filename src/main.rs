use reqwest;
use scraper::Selector;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the absolute URL of the IMDb trailers page
    let url = "https://www.imdb.com/chart/top/?ref_=nv_mv_250";
    // Fetch the HTML content of the webpage
    let body = reqwest::get(url).await?.text().await?;

    // Parse the HTML content into a document
    let document = scraper::Html::parse_document(&body);

    // Define the CSS selector
    let selector = Selector::parse("#__next > main > div > div.ipc-page-content-container.ipc-page-content-container--center > section > div > div.ipc-page-grid.ipc-page-grid--bias-left > div > ul").unwrap();

    // Iterate over elements matching the CSS selector
    for (index, element) in document.select(&selector).enumerate() {
        // Extract the text content from each element and collect it into a String
        let title = element.text().collect::<String>();

        // Print each movie title with a number and a bullet point
        println!("{}. {}", index + 1, title);
    }

    Ok(())
}
