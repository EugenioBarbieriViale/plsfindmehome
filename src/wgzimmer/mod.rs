use crate::WGQuery;
use handle_data::write_to_csv;

use futures::future::join_all;
use rand::random_range;
use scraper::{Html, Selector};
use std::path::Path;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

mod apply;
pub mod handle_data;

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

    let num_pages = get_num_pages(driver).await?;
    let mut data: Vec<Juice> = vec![];

    for i in 0..num_pages {
        println!("--- Extracting data from page {}/{} ---", i, num_pages);

        let j = Juice::extract(driver).await?;
        data.push(j);
        load_next_page(driver).await?;
    }

    match write_to_csv(path, data) {
        Ok(_) => {
            println!("Data successully saved to {:?}", path);
        }
        Err(e) => {
            println!("Could not write data to {:?}: {:?}", path, e);
        }
    }

    Ok(())
}

async fn search<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
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

async fn get_num_pages(driver: &WebDriver) -> WebDriverResult<usize> {
    sleep(Duration::from_secs(rnd())).await;
    let pages_str = driver
        .find(By::Css("span[class='counter']"))
        .await?
        .inner_html()
        .await?;

    match pages_str.find('/') {
        Some(n) => Ok(pages_str[(n + 1)..].trim().parse().unwrap()),
        None => {
            panic!("Could not get number of pages");
        }
    }
}

async fn load_next_page(driver: &WebDriver) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd() * 2)).await;
    driver
        .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;

    println!("Loading next page...");
    sleep(Duration::from_secs(rnd())).await;

    let next_elem = driver.find(By::Css("a[class='next']")).await?;
    let next_page_link = next_elem.attr("href").await?.unwrap();

    driver
        .execute(
            format!("open('{}', target='_self')", next_page_link),
            vec![],
        )
        .await?;

    println!("Next page has been loaded.");

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
    async fn extract(driver: &WebDriver) -> WebDriverResult<Self> {
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

            let link = String::from(
                fragment
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap(),
            );

            println!("Inserat {}", counter);
            counter += 1;

            prices.push(price);
            positions.push(position);
            dates.push(date);
            periods.push(period);
            links.push(link);
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
