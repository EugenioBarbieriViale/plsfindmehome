use crate::wgzimmer::rnd;

use scraper::{Html, Selector};
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

pub async fn click_consent_button(driver: &WebDriver) -> WebDriverResult<()> {
    let consent_button = driver.find(By::Css("p[class='fc-button-label']")).await;

    match consent_button {
        Ok(b) => b.click().await?,
        Err(_) => {
            eprintln!("No consent button to click");
        }
    }
    Ok(())
}

pub async fn get_num_pages(driver: &WebDriver) -> WebDriverResult<usize> {
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

pub async fn load_next_page(driver: &WebDriver) -> WebDriverResult<()> {
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

pub async fn goto_link(driver: &WebDriver, link: &String) -> WebDriverResult<()> {
    driver
        .execute(format!("open('{}', target='_self')", link), vec![])
        .await?;
    Ok(())
}

pub fn get_link(wg: &String) -> Option<String> {
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

pub fn get_price(wg: &String) -> Option<String> {
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

pub async fn back_to_list(driver: &WebDriver) -> WebDriverResult<()> {
    let back_elem = driver.find(By::Css("a[class='back']")).await?;
    let back_page_link = back_elem.attr("href").await?.unwrap();
    goto_link(driver, &back_page_link).await?;
    Ok(())
}
