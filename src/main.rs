use crate::wgzimmer::handle_data::handle_files;
use crate::wgzimmer::scrape;

use dotenv::dotenv;
use std::env;
use thirtyfour::prelude::*;

mod wgzimmer;

struct WGQuery<'a> {
    price_min: usize,
    price_max: usize,
    wg_state: &'a String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap();

    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("start-maximized")?;
    caps.add_arg("enable-automation")?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--disable-browser-side-navigation")?;
    caps.add_arg("--disable-gpu")?;
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

    let path = handle_files();
    scrape(&path, &driver, &q).await?;

    driver.quit().await?;

    Ok(())
}
