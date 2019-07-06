use std::{error::Error, path::PathBuf};
use table_extract;

#[derive(Default, Debug)]
pub struct Parkrun {
    url: String,
    path: PathBuf,
    records: Vec<Record>,
}

#[derive(Default, Debug)]
pub struct Record {
    position: String,
    parkrunner: String,
    time: String,
    category: String,
    age_grade: String,
    gender: String,
}

impl Parkrun {
    pub fn new(url: String, path: PathBuf) -> Self {
        Parkrun {
            url,
            path,
            records: Vec::with_capacity(300),
        }
    }

    pub fn orchestrate(&mut self) -> Result<(), Box<Error>> {
        self.download()?;
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<Error>> {
        Ok(())
    }

    pub fn download(&mut self) -> Result<(), Box<Error>> {
        let body = reqwest::get(&self.url)?.text()?;

        if let Some(data) = table_extract::Table::find_by_id(&body, "results") {
            for row in &data {
                let slice = row.as_slice();

                let rec = Record {
                    position: slice[0].clone(),
                    parkrunner: slice[1].clone(),
                    time: slice[2].clone(),
                    category: slice[3].clone(),
                    age_grade: slice[4].clone(),
                    gender: slice[5].clone(),
                };

                self.records.push(rec);
            }

            return Ok(());
        }

        Err("Unable to retrieve/save results.")?
    }
}
