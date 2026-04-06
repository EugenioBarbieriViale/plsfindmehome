use crate::Query;
use chromiumoxide::Browser;

pub async fn browse<'a>(
    browser: &Browser,
    url: &String,
    q: &Query<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    let page = browser.new_page(url).await?;
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

    println!("Browsing headless...");
    let html = page.wait_for_navigation().await?.content().await?;

    println!("{:?}", html);

    Ok(())
}
