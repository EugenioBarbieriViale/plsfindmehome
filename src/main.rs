use crate::wg_zimmer::scrape;
use chrono::Utc;
use dotenv::dotenv;
use std::env;
use std::fs::{File, create_dir};
use std::path::{Path, PathBuf};
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

    let path = handle_files();
    scrape(&path, &driver, &q).await?;

    driver.quit().await?;

    Ok(())
}

fn handle_files() -> PathBuf {
    let dir_path = env::var("DATA_PATH").unwrap().to_owned();
    // let csv_file: &String = &env::var("CSV_FILE").unwrap();
    let csv_file = format!("{}.csv", Utc::now().to_string());

    match create_dir(Path::new(&dir_path)) {
        Ok(_) => (),
        Err(_) => {
            println!("Directory {} already exists, not creating.", dir_path);
        }
    };

    // dir_path.push_str(csv_file);
    let dir_path = format!("{dir_path}{csv_file}");
    let path = Path::new(&dir_path);

    match File::create(path) {
        Ok(_) => (),
        Err(_) => {
            println!("File {:?} already exists, not creating.", path);
        }
    };

    path.to_owned()
}
