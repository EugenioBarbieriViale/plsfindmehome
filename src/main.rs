mod wg_zimmer;

use thirtyfour::prelude::*;

use crate::wg_zimmer::scrape;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless")?;
    let driver = WebDriver::new("http://localhost:32863", caps).await?;

    let url = "https://www.wgzimmer.ch/wgzimmer/search/mate.html";
    driver.goto(url).await?;

    match scrape(&driver).await {
        Ok(v) => v,
        Err(_e) => (),
    };

    driver.quit().await?;

    Ok(())
}
