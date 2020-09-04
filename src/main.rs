mod api;
mod artists;

use structopt::StructOpt;
use exitfailure::{ExitFailure};
use crate::artists::ArtistInfo;

#[derive(StructOpt)]
struct Cli {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let result = ArtistInfo::get(args.name).await?;
    println!("INFO: {:?}", result);
    Ok(())
}
