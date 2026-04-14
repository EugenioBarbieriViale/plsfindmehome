use crate::wg_zimmer::Juice;

use chrono::Utc;
use std::env;
use std::fs::{File, create_dir};
use std::path::{Path, PathBuf};

pub fn handle_files() -> PathBuf {
    let dir_path = env::var("DATA_PATH").unwrap().to_owned();
    let csv_file = format!("{}.csv", Utc::now().to_string());

    match create_dir(Path::new(&dir_path)) {
        Ok(_) => (),
        Err(_) => {
            println!("Directory {} already exists, not creating.", dir_path);
        }
    };

    let dir_path = format!("{dir_path}{csv_file}");
    let path = Path::new(&dir_path);

    File::create(path).unwrap();

    path.to_owned()
}

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
