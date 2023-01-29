use reqwest::{Error, Response};

pub struct Url<'a> {
    pub url: &'a str,
}

impl Url<'_> {
    pub async fn fetch<'a>(&self) -> Result<Response, Error> {
        reqwest::get(self.url).await
    }
}
