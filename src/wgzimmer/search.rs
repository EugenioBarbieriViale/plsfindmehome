use crate::WGQuery;
use crate::wgzimmer::rnd;

use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

pub async fn perform_search<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd())).await;
    driver.refresh().await?;

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute(
            format!(
                "var el = document.querySelector(\"select[name='priceMin']\");
                 el.value = '{}';
                 el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                 el.dispatchEvent(new Event('input', {{ bubbles: true }}));",
                query.price_min
            ),
            vec![],
        )
        .await?;
    println!("Minimum price set to {}.", query.price_min);

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute(
            format!(
                "var el = document.querySelector(\"select[name='priceMax']\");
                 el.value = '{}';
                 el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                 el.dispatchEvent(new Event('input', {{ bubbles: true }}));",
                query.price_max
            ),
            vec![],
        )
        .await?;
    println!("Maximum price set to {}.", query.price_max);

    sleep(Duration::from_secs(rnd())).await;
    let wg_state_button = driver
        .find(By::Css(format!(
            "span[class='stateShortcut'][data-state='{}']",
            query.wg_state
        )))
        .await?;
    wg_state_button.click().await?;
    println!("Wg state set to {}.", query.wg_state);

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;

    println!("Searching...");
    sleep(Duration::from_secs(rnd())).await;
    let search_button = driver.find(By::Css("input[value='Search']")).await;
    match search_button {
        Ok(v) => v.click().await?,
        Err(_) => {
            let b = driver.find(By::Css("input[value='Suchen']")).await?;
            b.click().await?;
        }
    }

    Ok(())
}
