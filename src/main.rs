mod clean;
mod parkrun;
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
}

fn main() {
    let opt = Opt::from_args();
    let mut pr = parkrun::Parkrun::new(opt.url);

    match pr.orchestrate() {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to complete \"{}\"", e),
    }
}
