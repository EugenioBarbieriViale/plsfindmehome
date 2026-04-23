use crate::WGQuery;
use crate::wgzimmer::rnd;

use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

pub async fn perform_search<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
    sleep(Duration::from_secs(rnd())).await;
    driver.refresh().await?;

    perform_actions(driver).await.unwrap_or_else(|err| {
        eprintln!("Could not perform actions: {err}");
    });

    sleep(Duration::from_secs(rnd())).await;
    make_query(driver, query).await?;

    sleep(Duration::from_secs(rnd())).await;
    driver
        .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;

    // let start_search = driver
    //     .find(By::Css("input[type='hidden'][name='startSearch']"))
    //     .await
    //     .unwrap();
    // println!("{}", start_search.text().await?);
    // function submitForm() {
    //     grecaptcha.execute(siteKey, {action: '//form///wgzimmer/search/mate/submitForm'})
    //         .then(function (token) {
    //             document.getElementById('g-recaptcha-response').value = token;
    //             document.searchMateForm.submit();
    //         });
    // }

    sleep(Duration::from_secs(rnd())).await;
    press_search_btn(driver).await?;

    Ok(())
}

async fn make_query<'a>(driver: &WebDriver, query: &WGQuery<'a>) -> WebDriverResult<()> {
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

    Ok(())
}

async fn press_search_btn(driver: &WebDriver) -> WebDriverResult<()> {
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

async fn perform_actions(driver: &WebDriver) -> WebDriverResult<()> {
    let selectors = driver.find_all(By::Css("div[class='selector']")).await?;
    for s in selectors {
        sleep(Duration::from_secs(rnd())).await;
        let offset = 10 + (-1 as i64).pow(rnd() as u32);

        driver
            .action_chain_with_delay(None, Some(Duration::from_secs(rnd())))
            .move_to_element_with_offset(&s, offset, -offset)
            .click()
            .perform()
            .await?;
    }

    sleep(Duration::from_secs(rnd())).await;
    let e = driver
        .find(By::Css("span[class='title small-block']"))
        .await?;

    driver
        .action_chain_with_delay(None, Some(Duration::from_secs(rnd())))
        .move_to_element_center(&e)
        .click()
        .perform()
        .await?;

    Ok(())
}
