mod parkrun;

use std::{env, ffi::OsStr, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// The URL containing the park run results.
    #[structopt(
        name = "parkrun url",
        long = "url",
        short = "u",
        raw(required = "true")
    )]
    url: String,

    /// Output path, or current working directory if not specified or - provided.
    #[structopt(
        name = "output path",
        short = "o",
        long = "output",
        default_value = "-",
        parse(from_os_str = "parse_output_directory")
    )]
    output: PathBuf,
}

fn parse_output_directory(src: &OsStr) -> PathBuf {
    if src == "-" {
        return env::current_dir().expect("Unable to access current working directory.");
    }

    PathBuf::from(src)
}

fn main() {
    let opt = Opt::from_args();
    let mut pr = parkrun::Parkrun::new(opt.url, opt.output);

    match pr.orchestrate() {
        Ok(_) => println!("Extract complete"),
        Err(e) => println!("Failed to complete {:?}", e),
    }
}
