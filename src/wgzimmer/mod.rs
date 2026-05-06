use crate::Application;
use crate::WGQuery;

use apply::send_appl;
use search::perform_search;
use utils::*;

use futures::future::join_all;
use scraper::Html;
use std::process;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

mod apply;
pub mod checkpoint;
mod search;
mod utils;

pub async fn scrape<'a>(
    driver: &WebDriver,
    query: &WGQuery<'a>,
    appl: &Application,
    all_links: &Vec<String>,
    collected: &mut Option<Vec<Wg>>,
) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd())).await;
    driver.refresh().await?;

    apply_stealth(driver).await.unwrap_or_else(|err| {
        eprintln!("Could not apply stealth, exiting: {err}");
        process::exit(1);
    });

    sleep(Duration::from_secs(rnd())).await;
    click_consent_button(driver).await?;

    sleep(Duration::from_secs(rnd())).await;
    perform_search(driver, query).await?;

    sleep(Duration::from_secs(1)).await;
    let num_pages = match get_num_pages(driver).await {
        Ok(n) => n,
        Err(_) => {
            println!("Found only one page.");
            1
        }
    };

    let mut data: Vec<Wg> = vec![];

    for i in 0..num_pages {
        println!("--- Page {}/{} ---", i, num_pages);
        match scrape_page(driver, appl, all_links, &mut data).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("An error occured, skipping page: {}", e);
                continue;
            }
        };
    }

    *collected = Some(data);

    Ok(())
}

async fn scrape_page(
    driver: &WebDriver,
    appl: &Application,
    all_links: &Vec<String>,
    data: &mut Vec<Wg>,
) -> WebDriverResult<()> {
    // unnecesary long, could just retrive price and link directly - minor fix (but too lazy)
    let wgs = driver
        .find_all(By::Css("li[class='search-result-entry search-mate-entry']"))
        .await
        .expect("Could not get all links.");

    let size = wgs.len();
    println!("{} wgs found.", size);

    let futures = wgs.iter().map(|wg| wg.outer_html());
    let str_wgs: Vec<_> = join_all(futures)
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    let mut count = 1;
    for wg in str_wgs {
        let f = Html::parse_fragment(&wg);

        let Some(price) = get_price(&f) else {
            eprintln!("Could not get price, skipping");
            continue;
        };
        let Some(link) = get_link(&f) else {
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

        let wg_info: Wg = match Wg::extract_info(&driver, &price, &link).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not extract info of wg, skipping: {e}");
                continue;
            }
        };
        println!("{count}/{size}: {:?}", wg_info);

        if data.contains(&wg_info) {
            println!("This wg is a duplicate, skipping: {link}");
            continue;
        }

        println!("Sending application...");
        sleep(Duration::from_secs(appl.wait_time)).await;
        if let Err(err) = send_appl(driver, appl).await {
            eprintln!("Could not send application, skipping: {err}");
            continue;
        }
        println!("Application sent.");

        data.push(wg_info);
        count += 1;

        sleep(Duration::from_secs(rnd())).await;
        if let Err(err) = back_to_list(driver).await {
            eprintln!("Could not go back to the list, exiting: {err}");
            return Ok(());
        };
    }

    sleep(Duration::from_secs(rnd())).await;
    match load_next_page(driver).await {
        Ok(_) => (),
        Err(_) => eprintln!("Could not load next page."),
    }

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
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
