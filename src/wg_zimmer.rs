use crate::Query;
use chromiumoxide::Browser;
use scraper::{Html, Selector};
use tokio::time::{Duration, sleep};

pub async fn browse<'a>(
    browser: &Browser,
    url: &String,
    q: &Query<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    let page = browser.new_page(url).await?;

    page.evaluate_on_new_document(
        r#"
    Object.defineProperty(navigator, 'webdriver', { get: () => false });
"#,
    )
    .await?;

    println!("Browsing headless...");

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

    match page.find_element("input[value='Search']").await {
        Ok(el) => {
            el.click().await?;
        }
        Err(_) => {
            sleep(Duration::from_secs(1)).await;
            page.find_element("input[value='Suchen']")
                .await?
                .click()
                .await?;
        }
    }

    println!("Waiting for page to load...");
    sleep(Duration::from_secs(3)).await;

    println!("New page loaded!");
    let html = page.wait_for_navigation().await?.content().await?;

    extract_juice(&html);

    Ok(())
}

fn extract_juice(html: &String) {
    let document = Html::parse_document(html);
    let links = Selector::parse("href").unwrap();
    let thumb_state = Selector::parse("span[class='thumbState']").unwrap();

    println!("Extracting data...");
    for elem in document.select(&links) {
        println!("{}", elem.html());
    }
}
