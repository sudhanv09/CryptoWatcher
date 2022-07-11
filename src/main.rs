use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Target currency
    vs_currency: String,

    /// Id of the crypto
    ids: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeckoMarket {
    vs_currency: String,
    ids: String,
    order: String,
    price_change_percentage: String,
}

#[derive(Debug, Deserialize)]
struct MarketOut {
    name: String,
    current_price: f32,
    total_volume: i32,
    high_24h: f32,
    low_24h: f32,
    price_change_24h: f32,
    price_change_percentage_24h: f32,
    ath: f32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let base_url = "https://api.coingecko.com/api/v3/coins/markets";

    let query = GeckoMarket {
        vs_currency: args.vs_currency,
        ids: args.ids,
        order: "market_cap_asc".to_string(),
        price_change_percentage: "7d".to_string(),
    };

    let client = reqwest::Client::new();
    let gecko_client = client
        .get(base_url)
        .query(&query)
        .send()
        .await
        .expect("unable to resolve")
        .text()
        // .json::<MarketOut>()
        .await
        .expect("failed to get payload");

    // println!("{}", gecko_client);

    let [v] = serde_json::from_str::<[MarketOut; 1]>(&gecko_client).unwrap();

    println!("Name: {:?}\n Price: {:?}", v.name, v.current_price);
}
