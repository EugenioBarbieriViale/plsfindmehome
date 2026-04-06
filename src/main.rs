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

    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    // let (browser, mut handler) =
    //     Browser::launch(BrowserConfig::builder().new_headless_mode().build()?).await?;

    let handle = tokio::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    browse(&browser, &url, &q).await?;

    handle.await?;
    Ok(())
}
