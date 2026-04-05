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

    // let (browser, mut handler) =
    //     Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().new_headless_mode().build()?).await?;

    let handle = tokio::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page(&url).await?;
    page.wait_for_navigation().await?;

    page.evaluate(format!(
        "document.querySelector(\"select[name='priceMin']\").value = '{}'",
        &q.price_min.to_string()
    ))
    .await?;

    page.evaluate(format!(
        "document.querySelector(\"select[name='priceMax']\").value = '{}'",
        &q.price_max.to_string()
    ))
    .await?;

    let state_selector = format!("span[class='stateShortcut'][data-state='{}']", q.wg_state);
    page.find_element(state_selector).await?.click().await?;

    page.wait_for_navigation().await?;

    page.find_element("input[value='Suchen']")
        .await?
        .click()
        .await?;

    let html = page.wait_for_navigation().await?.content().await?;
    println!("{:?}", html);

    handle.await?;

    Ok(())
}
