use crate::wgzimmer::handle_data::handle_files;
use crate::wgzimmer::scrape;

use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;
use thirtyfour::prelude::*;

mod wgzimmer;

struct WGQuery<'a> {
    price_min: usize,
    price_max: usize,
    wg_state: &'a String,
}

struct Application {
    name: String,
    email: String,
    msg: String,
    wait_time: u64,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap();

    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("start-maximized")?;
    // caps.add_arg("enable-automation")?;
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

    let msg = read_to_string(env::var("MSG_TXT_PATH").unwrap()).unwrap();
    let a = Application {
        name: env::var("NAME").unwrap(),
        email: env::var("EMAIL").unwrap(),
        msg,
        wait_time: 5,
    };

    let path = handle_files();
    scrape(&path, &driver, &q, &a).await?;

    driver.quit().await?;

    Ok(())
}
