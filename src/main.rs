use crate::wg_zimmer::scrape;
use dotenv::dotenv;
use std::env;
use std::path::Path;
use thirtyfour::prelude::*;

mod handle_csv;
mod wg_zimmer;

struct WGQuery<'a> {
    price_min: usize,
    price_max: usize,
    wg_state: &'a String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap();

    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless")?;
    let driver = WebDriver::new(format!("http://localhost:{}", port), caps).await?;

    let url = env::var("URL").unwrap();
    driver.goto(url).await?;

    let wg_states: Vec<String> = vec![
        "zurich-stadt".to_string(),
        "zurich-lake".to_string(),
        "zurich".to_string(),
        "zurich-oberland".to_string(),
    ];

    let q = WGQuery {
        price_min: 300,
        price_max: 800,
        wg_state: &wg_states[0],
    };

    let v = env::var("CSV_PATH").unwrap();
    let path = Path::new(&v);
    scrape(&path, &driver, &q).await?;

    driver.quit().await?;

    Ok(())
}
