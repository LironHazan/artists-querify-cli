use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Cli {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist { // using camelCase instead of snake_case due to the API
    strArtist: String,
    intFormedYear: String,
    strBiographyEN: String,
    strGenre: String,
    intMembers: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtistInfo {
    artists: Vec<Artist>,
}

impl ArtistInfo {
    async fn get(name: String) -> Result<Self, ExitFailure> {
        let url = format!("https://www.theaudiodb.com/api/v1/json/1/search.php?s=");
        let new_url = [url, name].join("\n");
        let result = reqwest::get(&new_url).await?
            .json::<ArtistInfo>()
            .await?;
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let result = ArtistInfo::get(args.name).await?;
    println!("INFO: {:?}", result);
    Ok(())
}
