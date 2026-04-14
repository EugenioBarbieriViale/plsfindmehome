use crate::wgzimmer::Juice;
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

async fn apply_all(driver: &WebDriver, data: &Vec<Juice>) -> WebDriverResult<()> {
    println!("Starting application process.");
    sleep(Duration::from_secs(5)).await;

    for j in data {
        for i in 0..j.size {
            sleep(Duration::from_secs(5)).await;
            apply(driver, &j.links[i]).await?;
        }
    }
    Ok(())
}

async fn apply(driver: &WebDriver, link: &String) -> WebDriverResult<()> {
    goto_link(driver, link).await?;
    open_tent(driver).await?;

    Ok(())
}

async fn goto_link(driver: &WebDriver, link: &String) -> WebDriverResult<()> {
    driver
        .execute(format!("open('{}', target='_self')", link), vec![])
        .await?;
    Ok(())
}

async fn open_tent(driver: &WebDriver) -> WebDriverResult<()> {
    sleep(Duration::from_secs(5)).await;

    let elem = driver.find(By::Css("a[class='small-link']")).await?;
    let js_box = elem.attr("onclick").await?.unwrap();

    driver.execute(js_box, vec![]).await?;
    Ok(())
}
