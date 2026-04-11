use crate::wg_zimmer::Juice;
use std::path::Path;

pub fn write_to_csv(path: &Path, data: &Juice) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(&["price", "position", "date", "period", "link"])?;
    for i in 0..data.size {
        let price = data.prices[i].to_string();
        wtr.write_record(&[
            price.as_str(),
            &data.positions[i],
            &data.dates[i],
            &data.periods[i],
            &data.links[i],
        ])?;
    }
    wtr.flush()?;

    Ok(())
}
