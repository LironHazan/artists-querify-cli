use exitfailure::{ExitFailure};
use crate::artists::{ArtistInfo};

pub async fn search(name: String) -> Result<ArtistInfo, ExitFailure> {
    let url = format!("https://www.theaudiodb.com/api/v1/json/1/search.php?s={}", name);
    let res = reqwest::get(&url).await?
        .json::<ArtistInfo>()
        .await?;
    Ok(res)
}
