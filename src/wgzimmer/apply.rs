use crate::Application;
use crate::wgzimmer::{Direction, rnd, scroll_to};
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

pub async fn send_appl(driver: &WebDriver, a: &Application) -> WebDriverResult<()> {
    open_tent(driver).await?;

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute(
            format!(
                "var el = document.querySelector(\"input[id='senderName']\");
                 el.value = '{}';
                 el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                 el.dispatchEvent(new Event('input', {{ bubbles: true }}));",
                a.name
            ),
            vec![],
        )
        .await?;

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute(
            format!(
                "var el = document.querySelector(\"input[id='senderEmail']\");
                 el.value = '{}';
                 el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                 el.dispatchEvent(new Event('input', {{ bubbles: true }}));",
                a.email
            ),
            vec![],
        )
        .await?;

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute(
            format!(
                "var el = document.querySelector(\"textarea[id='senderText']\");
                 el.value = {};
                 el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                 el.dispatchEvent(new Event('input', {{ bubbles: true }}));",
                a.msg
            ),
            vec![],
        )
        .await?;

    sleep(Duration::from_secs(1)).await;
    submit_appl(driver).await?;

    Ok(())
}

async fn open_tent(driver: &WebDriver) -> WebDriverResult<()> {
    let elem = driver.find(By::Css("a[class='small-link']")).await?;
    let js_box = elem.attr("onclick").await?.unwrap();

    driver.execute(js_box, vec![]).await?;
    Ok(())
}

async fn submit_appl(driver: &WebDriver) -> WebDriverResult<()> {
    let submit_btn = driver
        .find(By::Css("input[class='submit-inline-mail']"))
        .await?;
    scroll_to(driver, Direction::ToElement(submit_btn.clone())).await?;
    submit_btn.click().await?;
    Ok(())
}
