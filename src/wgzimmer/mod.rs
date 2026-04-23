use crate::Application;
use crate::WGQuery;

use apply::send_appl;
use handle_data::write_to_csv;
use search::perform_search;
use utils::*;

use futures::future::join_all;
use rand::random_range;
use std::path::Path;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

mod apply;
pub mod handle_data;
mod search;
mod utils;

pub async fn scrape<'a>(
    path: &Path,
    driver: &WebDriver,
    query: &WGQuery<'a>,
    appl: &Application,
    all_links: &Vec<String>,
) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd())).await;
    click_consent_button(driver).await?;

    sleep(Duration::from_secs(rnd())).await;
    perform_search(driver, query).await?;

    sleep(Duration::from_secs(rnd())).await;
    let num_pages = get_num_pages(driver).await?;

    let data = get_data(driver, appl, num_pages, all_links).await;

    match data {
        Some(data) => match write_to_csv(path, data) {
            Ok(_) => {
                println!("Data successully saved to {:?}", path);
            }
            Err(e) => {
                eprintln!("Could not write data to {:?}: {:?}", path, e);
            }
        },
        None => {
            println!("No data has been collected");
        }
    }

    Ok(())
}

async fn get_data(
    driver: &WebDriver,
    appl: &Application,
    num_pages: usize,
    all_links: &Vec<String>,
) -> Option<Vec<Vec<Wg>>> {
    let mut data: Vec<Vec<Wg>> = vec![];

    for i in 0..num_pages {
        println!("--- Page {}/{} ---", i, num_pages);
        let page_data = match scrape_page(driver, appl, all_links).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("An error occured, skipping page: {}", e);
                continue;
            }
        };
        data.push(page_data);
    }

    Some(data)
}

async fn scrape_page(
    driver: &WebDriver,
    appl: &Application,
    all_links: &Vec<String>,
) -> WebDriverResult<Vec<Wg>> {
    // unnecesary long, could just retrive price and link directly - minor fix
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
    let mut count = 1;

    for wg in str_wgs {
        let Some(price) = get_price(&wg) else {
            eprintln!("Could not get price, skipping");
            continue;
        };
        let Some(link) = get_link(&wg) else {
            eprintln!("Could not get link, skipping");
            continue;
        };

        if all_links.contains(&link) {
            println!("This wg has already been processed in the past, skipping: {link}");
            continue;
        }

        if let Err(err) = goto_link(driver, &link).await {
            eprintln!("Could not go to link {link}, skipping: {err}");
            continue;
        }

        sleep(Duration::from_secs(rnd())).await;

        let wg_info: Wg = match Wg::extract_info(&driver, &price, &link).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not extract info of wg, skipping: {e}");
                continue;
            }
        };
        println!("{count}/{size}: {:?}", wg_info);

        sleep(Duration::from_secs(appl.wait_time)).await;
        if let Err(err) = send_appl(driver, appl).await {
            eprintln!("Could not send_application, skipping: {err}");
            continue;
        }

        page_data.push(wg_info);
        count += 1;

        sleep(Duration::from_secs(rnd())).await;
        if let Err(err) = back_to_list(driver).await {
            eprintln!("Could not go back to the list, exiting: {err}");
            return Ok(page_data);
        };
    }

    sleep(Duration::from_secs(rnd() * 2)).await;
    load_next_page(driver).await?;

    Ok(page_data)
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

        let address = ps[1].text().await?;
        let place = ps[0].text().await?;

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

pub fn rnd() -> u64 {
    random_range(2..=5)
}
