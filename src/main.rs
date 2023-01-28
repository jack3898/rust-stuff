mod fetcher;

use fetcher::Url;
use rocket::tokio;

#[tokio::main]
async fn main() {
    let url = Url {
        url: "https://randomuser.me/api",
    };

    let text = url.fetch().await;

    println!("{:?}", text)
}
