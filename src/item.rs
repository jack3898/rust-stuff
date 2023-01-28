use serde::{Deserialize, Serialize};
use serde_json;

use crate::fetcher::Url;

pub struct Items<'a> {
    pub url: Url<'a>,
}

impl Items<'_> {
    pub async fn fetch(&self) -> OsrsItems {
        // I know, I know, unwrap hell here 😂
        // Still learning the ropes, will handle this properly sometime
        let response = &self.url.fetch().await.unwrap().text().await.unwrap();

        serde_json::from_str(response).unwrap()
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
