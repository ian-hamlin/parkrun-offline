use crate::html::Html;
use csv;
use serde::Serialize;
use std::error::Error;
use table_extract;

#[derive(Default, Debug)]
pub struct Parkrun {
    url: String,
    records: Vec<Record>,
}

#[derive(Default, Debug, Serialize)]
pub struct Record {
    position: String,
    parkrunner: String,
    time: String,
    category: String,
    age_grade: String,
    gender: String,
}

impl Parkrun {
    pub fn new(url: String) -> Self {
        Parkrun {
            url,
            records: Vec::with_capacity(300),
        }
    }

    pub fn orchestrate(&mut self) -> Result<(), Box<Error>> {
        self.download()?;
        self.save()?;
        Ok(())
    }

    pub fn download(&mut self) -> Result<(), Box<Error>> {
        let body = reqwest::get(&self.url)?.text()?;

        if let Some(data) = table_extract::Table::find_by_id(&body, "results") {
            for row in &data {
                let slice = row.as_slice();

                let rec = Record {
                    position: slice[0].clone(),
                    parkrunner: slice[1].remove_anchor(),
                    time: slice[2].clone(),
                    category: slice[3].remove_anchor(),
                    age_grade: slice[4].clone(),
                    gender: slice[5].clone(),
                };

                self.records.push(rec);
            }

            return Ok(());
        }

        Err(format!("Unable to retrieve results from {}", self.url))?
    }

    pub fn save(&self) -> Result<(), Box<Error>> {
        let out = std::io::stdout();
        let out = out.lock();
        let mut wtr = csv::Writer::from_writer(out);

        for rec in &self.records {
            wtr.serialize(rec)?;
        }

        wtr.flush()?;
        Ok(())
    }
}
