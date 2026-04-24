use crate::wgzimmer::Wg;

use chrono::Local;
use std::fs::{File, create_dir, read_dir};
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn init(dir_path: &String) -> PathBuf {
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

pub fn save(path: &Path, data: Vec<Wg>) -> Result<(), csv::Error> {
    File::create(&path).unwrap();
    println!("Created file {:?}.", path);

    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(&["price", "link", "address", "place", "from", "until"])?;
    for wg in data {
        wtr.write_record(&[wg.price, wg.link, wg.address, wg.place, wg.from, wg.until])?;
    }
    wtr.flush()?;

    Ok(())
}

pub fn get_all_links(dir_path: &String, col_index: usize) -> Result<Vec<String>, csv::Error> {
    let entries = read_dir(Path::new(dir_path))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut all_links = vec![];
    for e in entries {
        let mut rdr = csv::Reader::from_path(&e)?;
        for r in rdr.records() {
            all_links.push(r?.get(col_index).unwrap().to_string());
        }
    }

    Ok(all_links)
}
