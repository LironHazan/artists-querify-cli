use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use crate::api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist { // using camelCase instead of snake_case due to the API
strArtist: String,
    intFormedYear: String,
    strBiographyEN: String,
    strGenre: String,
    intMembers: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtistInfo {
    artists: Vec<Artist>,
}

impl ArtistInfo {
    pub async fn get(name: String) -> Result<Self, ExitFailure> {
        let result = api::search(name).await?;
        Ok(result)
    }
}
