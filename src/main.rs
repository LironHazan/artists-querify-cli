mod api;
mod artists;
mod view;

#[macro_use] extern crate prettytable;
use crate::artists::{ArtistInfo, Artist};
use structopt::StructOpt;
use exitfailure::{ExitFailure};

#[derive(StructOpt)]
struct Cli {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let info = ArtistInfo::get(args.name).await?;
    let artist = info.artists.first();
    Ok(view::output_artist_table(artist.unwrap()))
}
