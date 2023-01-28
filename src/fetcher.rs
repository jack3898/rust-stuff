pub struct Url<'a> {
    pub url: &'a str,
}

impl Url<'_> {
    pub async fn fetch<'a>(&self) -> String {
        let response = reqwest::get(self.url).await;

        match response {
            Result::Ok(ok) => format!("{:?}", ok),
            Result::Err(_) => String::from(""),
        }
    }
}
