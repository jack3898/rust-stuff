use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::url::Url;

pub struct Items<'a> {
    pub url: Url<'a>,
}

impl Items<'_> {
    pub async fn fetch(&self) -> Result<OsrsItems, Error> {
        let request = self.url.fetch().await;

        // Get the text from the response body
        let text = match request {
            Ok(response) => response.text().await,
            Err(error) => return Err(error),
        };

        // Convert the text to a Rust data structure
        let deserialized: OsrsItems = match text {
            Ok(text) => serde_json::from_str(text.as_str()).unwrap(),
            Err(error) => return Err(error),
        };

        Ok(deserialized)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OsrsItems {
    pub total: u32,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub icon_large: String,
}
