use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use crate::api;
use std::fmt;
use serde::export::Formatter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist { // using camelCase instead of snake_case due to the API
    pub strArtist: String,
    pub intFormedYear: String,
    pub strBiographyEN: String,
    pub strGenre: String,
    pub intMembers: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtistInfo {
    pub(crate) artists: Vec<Artist>,
}

impl ArtistInfo {
    pub async fn get(name: String) -> Result<Self, ExitFailure> {
        let result = api::search(name).await?;
        Ok(result)
    }
}
