use crate::wgzimmer::Wg;

use chrono::Local;
use std::env;
use std::fs::File;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

pub fn handle_files() -> PathBuf {
    let dir_path = env::var("DATA_PATH").unwrap().to_owned();
    let csv_file = format!("{}.csv", Local::now().to_string()).replace(" ", "_");

    match create_dir(Path::new(&dir_path)) {
        Ok(_) => (),
        Err(_) => {
            println!("Directory {} already exists, not creating.", dir_path);
        }
    };

    let dir_path = format!("{dir_path}{csv_file}");
    let path = Path::new(&dir_path);

    path.to_owned()
}

pub fn write_to_csv(path: &Path, data: Vec<Vec<Wg>>) -> Result<(), csv::Error> {
    File::create(&path).unwrap();
    println!("Created file {:?}.", path);

    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(&["price", "link", "address", "place", "from", "until"])?;
    for page_data in data {
        for wg in page_data {
            wtr.write_record(&[wg.price, wg.link, wg.address, wg.place, wg.from, wg.until])?;
        }
    }
    wtr.flush()?;

    Ok(())
}
