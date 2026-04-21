use crate::Application;
use crate::WGQuery;
use apply::send_appl;
use handle_data::write_to_csv;

use futures::future::join_all;
use rand::random_range;
use scraper::{Html, Selector};
use std::path::Path;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

mod apply;
pub mod handle_data;

pub async fn scrape<'a>(
    path: &Path,
    driver: &WebDriver,
    query: &WGQuery<'a>,
    appl: &Application,
) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd())).await;
    let consent_button = driver.find(By::Css("p[class='fc-button-label']")).await;

    match consent_button {
        Ok(b) => b.click().await?,
        Err(_) => {
            eprintln!("No consent button to click");
        }
    }

    search(driver, query).await?;
    sleep(Duration::from_secs(3)).await;

    sleep(Duration::from_secs(rnd())).await;
    let num_pages = get_num_pages(driver).await?;

    let mut data: Vec<Vec<Wg>> = vec![];
    for i in 0..num_pages {
        println!("--- Page {}/{} ---", i, num_pages);
        let page_data = match scrape_page(driver, appl).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("An error occured, skipping page: {}", e);
                continue;
            }
        };
        data.push(page_data);
    }

    match write_to_csv(path, data) {
        Ok(_) => {
            println!("Data successully saved to {:?}", path);
        }
        Err(e) => {
            eprintln!("Could not write data to {:?}: {:?}", path, e);
        }
    }

    Ok(())
}

async fn scrape_page(driver: &WebDriver, appl: &Application) -> WebDriverResult<Vec<Wg>> {
    let wgs = driver
        .find_all(By::Css("li[class='search-result-entry search-mate-entry']"))
        .await?;

    let size = wgs.len();
    println!("{} wgs found.", size);

    let futures = wgs.iter().map(|wg| wg.outer_html());
    let str_wgs: Vec<_> = join_all(futures)
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    let mut page_data: Vec<Wg> = vec![];
    for wg in str_wgs {
        let Some(price) = get_price(&wg) else {
            eprintln!("Could not get price, skipping");
            continue;
        };
        let Some(link) = get_link(&wg) else {
            eprintln!("Could not get link, skipping");
            continue;
        };

        goto_link(driver, &link).await?;
        sleep(Duration::from_secs(rnd())).await;

        let wg_info: Wg = Wg::extract_info(&driver, &price, &link).await?;
        println!("{:?}", wg_info);

        page_data.push(wg_info);

        sleep(Duration::from_secs(appl.wait_time)).await;
        send_appl(driver, appl).await?;

        sleep(Duration::from_secs(rnd())).await;
        back_to_list(driver).await?;
    }

    sleep(Duration::from_secs(rnd() * 2)).await;
    load_next_page(driver).await?;

    Ok(page_data)
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
    driver
        .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;

    println!("Loading next page...");
    sleep(Duration::from_secs(rnd())).await;

    let next_elem = driver.find(By::Css("a[class='next']")).await?;
    let next_page_link = next_elem.attr("href").await?.unwrap();

    goto_link(driver, &next_page_link).await?;

    println!("Next page has been loaded.");

    Ok(())
}

async fn goto_link(driver: &WebDriver, link: &String) -> WebDriverResult<()> {
    driver
        .execute(format!("open('{}', target='_self')", link), vec![])
        .await?;
    Ok(())
}

fn get_link(wg: &String) -> Option<String> {
    let fragment = Html::parse_fragment(wg);
    let link = String::from(
        fragment
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap(),
    );

    Some(link)
}

fn get_price(wg: &String) -> Option<String> {
    let fragment = Html::parse_fragment(wg);

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

    Some(price)
}

#[derive(Debug)]
pub struct Wg {
    price: String,
    link: String,
    address: String,
    place: String,
    from: String,
    until: String,
}

impl Wg {
    async fn extract_info(
        driver: &WebDriver,
        price: &String,
        link: &String,
    ) -> WebDriverResult<Self> {
        let container = driver
            .find(By::Css("div[class='wrap col-wrap date-cost']"))
            .await?;

        let ps = container.find_all(By::Tag("p")).await?;

        let from = ps[0].text().await?;
        let until = ps[1].text().await?;

        let from = from.trim_start_matches("Ab dem").trim().to_string();
        let until = until.trim_start_matches("Bis").trim().to_string();

        let container = driver
            .find(By::Css("div[class='wrap col-wrap adress-region']"))
            .await?;

        let ps = container.find_all(By::Tag("p")).await?;

        let address = ps[0].text().await?;
        let place = ps[1].text().await?;

        let address = address
            .trim_start_matches("Adresse")
            .trim()
            .replace("\n", "")
            .to_string();

        let place = place
            .trim_start_matches("Ort")
            .trim()
            .replace("\n", "")
            .to_string();

        Ok(Self {
            price: price.to_owned(),
            link: link.to_owned(),
            address,
            place,
            from,
            until,
        })
    }
}

async fn back_to_list(driver: &WebDriver) -> WebDriverResult<()> {
    let back_elem = driver.find(By::Css("a[class='back']")).await?;
    let back_page_link = back_elem.attr("href").await?.unwrap();
    goto_link(driver, &back_page_link).await?;
    Ok(())
}

fn rnd() -> u64 {
    random_range(1..=4)
}
