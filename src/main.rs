mod osrs_item;
mod url;

use osrs_item::Items;
use rand::Rng;
use rocket::tokio;
use url::Url;

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let page = rng.gen_range(0..10);
    let alpha = rng.gen_range(b'a'..b'z') as char;
    let url_str = format!("https://secure.runescape.com/m=itemdb_oldschool/api/catalogue/items.json?category=1&alpha={}&page={}", alpha, page);

    let url = Url {
        url: url_str.as_str(),
    };

    let test = Items { url };
    let res = test.fetch().await;
    let items = &res.unwrap().items;
    let item_index = rng.gen_range(0..items.len() - 1);

    println!(
        "I found a {} and the description is \"{}\".",
        items[item_index].name, items[item_index].description
    );
    println!("Here's a pic: {}", items[item_index].icon_large);
}
