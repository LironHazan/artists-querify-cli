use exitfailure::{ExitFailure};
use crate::artists::{ArtistInfo};

pub async fn search(name: String) -> Result<ArtistInfo, ExitFailure> {
    let mut url = format!("https://www.theaudiodb.com/api/v1/json/1/search.php?s=");
    url = [url, name].join("\n");
    let result = reqwest::get(&url).await?
        .json::<ArtistInfo>()
        .await?;
    Ok(result)

    // match make_request() {
    //     Err(e) => handler(e),
    //     Ok(_)  => return,
   // }
}
