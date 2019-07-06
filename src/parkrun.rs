use std::{error::Error, path::PathBuf, str::FromStr};
use table_extract;

#[derive(Default, Debug)]
pub struct Parkrun {
    url: String,
    path: PathBuf,
    records: Vec<Record>,
}

#[derive(Default, Debug)]
pub struct Record {
    pos: u16,
}

impl Parkrun {
    pub fn new(url: String, path: PathBuf) -> Self {
        Parkrun {
            url,
            path,
            records: Vec::with_capacity(300),
        }
    }

    pub fn download(&mut self) -> Result<(), Box<Error>> {
        let body = reqwest::get(&self.url)?.text()?;

        if let Some(data) = table_extract::Table::find_by_id(&body, "results") {
            for row in &data {
                let slice = row.as_slice();
                let position = u16::from_str(&slice[0]);
                let parkrunner = &slice[1];
                let time = &slice[2];
                let category = &slice[3];
                let age_grade = &slice[4];
                let gender = &slice[5];

                println!("{:?}", position);
            }

            return Ok(());
        }

        Err("Unable to retrieve/save results.")?
    }
}
