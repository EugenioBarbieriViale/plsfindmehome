use crate::WGQuery;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

pub async fn scrape<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
    driver
        .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;
    sleep(Duration::from_secs(1)).await;

    search(driver, query).await?;

    Ok(())
}

pub async fn search<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
    let _price_min = driver.execute(
        format!(
            "document.querySelector(\"select[name='priceMin']\").value = '{}'",
            query.price_min
        ),
        vec![],
    );

    let _price_max = driver.execute(
        format!(
            "document.querySelector(\"select[name='priceMax']\").value = '{}'",
            query.price_max
        ),
        vec![],
    );

    let wg_state_button = driver
        .find(By::Css(format!(
            "span[class='stateShortcut'][data-state='{}']",
            query.wg_state
        )))
        .await?;
    wg_state_button.click().await?;

    let search_button = driver.find(By::Css("input[value='Search']")).await;
    match search_button {
        Ok(v) => v.click().await?,
        Err(_) => {
            let b = driver.find(By::Css("input[value='Suchen']")).await?;
            b.click().await?;
        }
    }

    sleep(Duration::from_secs(3)).await;
    // implicitly waiting for new page to load
    // driver.find(By::Css("a[title='Neue Suche']")).await?;

    // Juice::extract(driver).await?;

    Ok(())
}

struct Juice {
    price: Vec<usize>,
    position: Vec<String>,
    from: Vec<String>,
    to: Vec<String>,
    link: Vec<String>,
}

impl Juice {
    pub async fn extract(driver: &WebDriver) -> WebDriverResult<Self> {
        let wg = driver
            .find_all(By::Css("li[class='search-result-entry search-mate-entry']"))
            .await?;

        println!("{:?}", wg[0]);
        println!("{}", wg.len());

        Ok(Self {
            price: vec![],
            position: vec![],
            from: vec![],
            to: vec![],
            link: vec![],
        })
    }
}
