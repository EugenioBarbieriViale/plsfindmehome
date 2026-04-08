// TODO:
// - implement wait_for_selector()
// - error matching for language differences DONE

mod wg_zimmer;

use crate::wg_zimmer::browse;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

struct Query<'a> {
    price_min: usize,
    price_max: usize,
    wg_state: &'a String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://www.wgzimmer.ch/en/wgzimmer/search/mate.html");

    let wg_states: Vec<String> = vec![
        "zurich-stadt".to_string(),
        "zurich-lake".to_string(),
        "zurich".to_string(),
        "zurich-oberland".to_string(),
    ];

    let q = Query {
        price_min: 200,
        price_max: 800,
        wg_state: &wg_states[0],
    };

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head()
            .window_size(1920, 1080)
            .build()?,
    )
    .await?;

    // let (browser, mut handler) =
    //     Browser::launch(BrowserConfig::builder().new_headless_mode().build()?).await?;

    // let (browser, mut handler) = Browser::launch(
    //     BrowserConfig::builder()
    //         .arg("--headless=old")
    //         .arg("--disable-blink-features=AutomationControlled")
    //         .arg("--no-sandbox")
    //         .arg("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36")
    //         .window_size(1920, 1080)
    //         .build()?,
    // )
    // .await?;

    let handle = tokio::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    browse(&browser, &url, &q).await?;

    handle.await?;
    Ok(())
}
