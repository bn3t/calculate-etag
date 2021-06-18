use calculate_etag::calculate_etag;
use calculate_etag::error::Result;

use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq, Eq)]
#[structopt(about, author)]
pub struct CalculateEtag {
    #[structopt(short = "s", long = "size", default_value = "8388608")]
    chunk_size: usize,
    /// Files to process
    #[structopt(required = true)]
    files: Vec<String>,
}

fn run(matches: CalculateEtag) -> Result<()> {
    for file in matches.files {
        let etag = calculate_etag(&file, matches.chunk_size)?;
        println!("{:<20} {}", etag, file);
    }

    Ok(())
}

fn main() {
    let matches = CalculateEtag::from_args();
    if let Err(e) = run(matches) {
        eprintln!("{}", e);
    }
}
