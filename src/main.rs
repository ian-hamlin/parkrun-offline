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

    /// A flag indicating if Unknown runners should be removed from the results, defaults to false if not supplied.
    #[structopt(name = "remove unknown", short = "r", long = "remove")]
    remove_unknown: bool,
}

fn main() {
    let opt = Opt::from_args();
    let mut pr = parkrun::Parkrun::new(opt.url, opt.remove_unknown);

    match pr.orchestrate() {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to complete \"{}\"", e),
    }
}
