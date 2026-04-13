use crate::wg_zimmer::Juice;
use std::path::Path;

pub fn write_to_csv(path: &Path, data: Vec<Juice>) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(&["price", "position", "date", "period", "link"])?;
    for juice in data {
        for i in 0..juice.size {
            let price = juice.prices[i].to_string();
            wtr.write_record(&[
                price.as_str(),
                &juice.positions[i],
                &juice.dates[i],
                &juice.periods[i],
                &juice.links[i],
            ])?;
        }
    }
    wtr.flush()?;

    Ok(())
}
