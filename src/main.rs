mod fetcher;
mod item;

use fetcher::Url;
use item::Items;
use rocket::tokio;

#[tokio::main]
async fn main() {
    let url = Url {
        url: "https://secure.runescape.com/m=itemdb_oldschool/api/catalogue/items.json?category=1&alpha=c&page=1",
    };

    let test = Items { url };

    let res = test.fetch().await;

    println!("{:?}", res.items[0].description)
}
