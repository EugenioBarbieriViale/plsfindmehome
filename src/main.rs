mod wg_zimmer;

use crate::wg_zimmer::scrape;
use thirtyfour::prelude::*;

struct WGQuery<'a> {
    price_min: usize,
    price_max: usize,
    wg_state: &'a String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless")?;
    let driver = WebDriver::new("http://localhost:32863", caps).await?;

    let url = "https://www.wgzimmer.ch/wgzimmer/search/mate.html";
    driver.goto(url).await?;

    let wg_states: Vec<String> = vec![
        "zurich-stadt".to_string(),
        "zurich-lake".to_string(),
        "zurich".to_string(),
        "zurich-oberland".to_string(),
    ];

    let q = WGQuery {
        price_min: 200,
        price_max: 800,
        wg_state: &wg_states[0],
    };

    scrape(&driver, &q).await?;

    // driver.quit().await?;

    Ok(())
}
