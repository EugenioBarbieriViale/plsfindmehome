use crate::WGQuery;
use crate::handle_csv::write_to_csv;
use futures::future::join_all;
use rand::random_range;
use scraper::{Html, Selector};
use std::path::Path;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

fn rnd() -> u64 {
    random_range(1..=4)
}

pub async fn scrape<'a>(
    path: &Path,
    driver: &WebDriver,
    query: &WGQuery<'a>,
) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd())).await;
    let consent_button = driver.find(By::Css("p[class='fc-button-label']")).await;
    match consent_button {
        Ok(b) => b.click().await?,
        Err(_) => {
            println!("No consent button to click");
        }
    }

    search(driver, query).await?;
    sleep(Duration::from_secs(3)).await;

    let j = Juice::extract(driver).await?;
    match write_to_csv(path, &j) {
        Ok(_) => {
            println!("Data successully saved in {:?}", path);
        }
        Err(e) => {
            println!("Could not write data to {:?}: {:?}", path, e);
        }
    }

    Ok(())
}

pub async fn search<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
    println!("Minimum price set to {}.", query.price_min);
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

    println!("Maximum price set to {}.", query.price_max);
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

    println!("Wg state set to {}.", query.wg_state);
    sleep(Duration::from_secs(rnd())).await;
    let wg_state_button = driver
        .find(By::Css(format!(
            "span[class='stateShortcut'][data-state='{}']",
            query.wg_state
        )))
        .await?;
    wg_state_button.click().await?;

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
    println!("Done.");

    Ok(())
}

pub struct Juice {
    pub size: usize,
    pub prices: Vec<usize>,
    pub positions: Vec<String>,
    pub dates: Vec<String>,
    pub periods: Vec<String>,
    pub links: Vec<String>,
}

impl Juice {
    pub async fn extract(driver: &WebDriver) -> WebDriverResult<Self> {
        let wgs = driver
            .find_all(By::Css("li[class='search-result-entry search-mate-entry']"))
            .await?;

        let size = wgs.len();
        println!("{} wgs found.", size);

        let futures = wgs.iter().map(|wg| wg.outer_html());
        let html_wgs: Vec<_> = join_all(futures)
            .await
            .into_iter()
            .collect::<Result<_, _>>()?;

        let mut prices: Vec<usize> = vec![];
        let mut positions: Vec<String> = vec![];
        let mut dates: Vec<String> = vec![];
        let mut periods: Vec<String> = vec![];
        let mut links: Vec<String> = vec![];

        let mut counter = 0;

        for wg in html_wgs {
            let fragment = Html::parse_fragment(&wg);

            let price_selector = Selector::parse("span[class='cost']").unwrap();
            let price = fragment
                .select(&price_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .parse()
                .unwrap();

            let pos_selector = Selector::parse("span[class='thumbState']").unwrap();
            let position = fragment
                .select(&pos_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>();

            let date_selector =
                Selector::parse("div.create-date.left-image-result strong").unwrap();
            let date = fragment
                .select(&date_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>();

            let period_selector = Selector::parse("span[class='from-date']").unwrap();
            let period = fragment
                .select(&period_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>();

            let link = fragment
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();

            println!("--- {} ---", counter);
            counter += 1;

            println!("{}", price);
            println!("{}", position);
            println!("{}", date);
            println!("{}", period);
            println!("{}", link);
            println!("");

            prices.push(price);
            positions.push(position);
            dates.push(date);
            periods.push(period);
            links.push(String::from(link));
        }

        Ok(Self {
            size,
            prices,
            positions,
            dates,
            periods,
            links,
        })
    }
}
