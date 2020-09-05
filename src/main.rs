mod api;
mod artists;
mod view;

#[macro_use] extern crate prettytable;
use crate::artists::ArtistInfo;
use structopt::StructOpt;
use exitfailure::{ExitFailure};

#[derive(StructOpt)]
struct Cli {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let result = ArtistInfo::get(args.name).await?;
    view::output_artist_table(result);
    Ok(())
}
