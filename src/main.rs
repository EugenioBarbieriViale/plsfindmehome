// TODO:
// - implement wait_for_selector()
// - error matching for language differences DONE

mod wg_zimmer;

use crate::wg_zimmer::browse;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use rand::seq::IndexedRandom;

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

    let proxies: Vec<String> = vec![
        "http://149.210.158.107:5001".to_string(),
        "http://147.182.255.208:443	".to_string(),
        "http://81.171.24.164:443".to_string(),
        "http://149.210.243.125:443	".to_string(),
        "http://52.140.7.131:443".to_string(),
        "http://52.178.94.61:443".to_string(),
        "http://124.156.223.252:4433".to_string(),
        "http://103.3.63.59:443".to_string(),
        "http://134.0.63.185:443".to_string(),
        "http://165.22.60.108:443".to_string(),
    ];

    let proxy_arg = format!(
        "--proxy-server={}",
        proxies.choose(&mut rand::rng()).unwrap()
    );
    println!("{}", proxy_arg);

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head()
            .window_size(1920, 1080)
            .arg(proxy_arg)
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
