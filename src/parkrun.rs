use crate::clean::Clean;
use csv;
use serde::Serialize;
use std::error::Error;
use table_extract;

#[derive(Default, Debug)]
pub struct Parkrun {
    url: String,
    records: Vec<Record>,
    remove_unknown: bool,
}

#[derive(Default, Debug, Serialize)]
pub struct Record {
    position: String,
    athlete_number: String,
    parkrunner: String,
    time: String,
    category: String,
    age_grade: String,
    age_grade_class: String,
    gender_position: String,
    gender: String,
    total_runs: String,
    first_timer: String,
    new_pb: String,
}

impl Parkrun {
    pub fn new(url: String, remove_unknown: bool) -> Self {
        Parkrun {
            url,
            records: Vec::with_capacity(1000),
            remove_unknown,
        }
    }

    pub fn orchestrate(&mut self) -> Result<(), Box<Error>> {
        self.download_and_format()?;
        self.save()?;
        Ok(())
    }

    pub fn download_and_format(&mut self) -> Result<(), Box<Error>> {
        let body = reqwest::get(&self.url)?.text()?;

        if let Some(data) = table_extract::Table::find_by_id(&body, "results") {
            for row in &data {
                let slice = row.as_slice();

                if slice.is_empty() {
                    // some parkruns (for example yakutskdokhsun) have a double
                    // header row which causes some issues.
                    continue;
                }

                let athlete_number = slice
                    .get(1)
                    .unwrap_or(&String::from(""))
                    .find_athlete_number();

                if self.remove_unknown && athlete_number.is_empty() {
                    continue;
                }

                let parkrunner = slice.get(1).unwrap_or(&String::from("")).remove_anchor();
                let percent_val = slice
                    .get(4)
                    .unwrap_or(&String::from(""))
                    .remove_percentage();

                let rec = Record {
                    position: slice.get(0).unwrap_or(&String::from("")).clone(),
                    athlete_number,
                    parkrunner,
                    time: slice.get(2).unwrap_or(&String::from("")).normalise_time(),
                    category: slice.get(3).unwrap_or(&String::from("")).remove_anchor(),
                    age_grade: percent_val.clone(),
                    age_grade_class: percent_val.normalise_age_grade(),
                    gender_position: slice.get(6).unwrap_or(&String::from("")).clone(),
                    gender: slice.get(5).unwrap_or(&String::from("")).clone(),
                    total_runs: slice.get(9).unwrap_or(&String::from("")).clone(),
                    first_timer: slice
                        .get(8)
                        .unwrap_or(&String::from(""))
                        .normalise_first_timer(),
                    new_pb: slice
                        .get(8)
                        .unwrap_or(&String::from(""))
                        .normalise_personal_best(),
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
