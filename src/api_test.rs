///Local testing
///This tests the API and sockets
//use reqwest::Error;
use reqwest::Client;
use serde_json::Value;
// use std::error::Error;

///Test the call of API once to ensure it is properly connected to the endpoint
pub async fn api_test_once() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api-futures.kucoin.com/api/v1/level2/depth20?symbol=ETHUSDTM";

    let response = client.get(url).send().await?;
    let mut res = String::new();

    if response.status().is_success() {
        res = response.text().await?;
        println!("{}", &res);
    } else {
        eprintln!("Failed to fetch data: {}", response.status());
    }

    let json_data: Value = serde_json::from_str(&res)?;

    let mut bids: Vec<(f64, i64)> = vec![];
    let mut asks: Vec<(f64, i64)> = vec![];


    // Parse bids & asks manually (for performance)
    if let Some(bid) = json_data["data"]["bids"].as_array() {
        println!("Top 5 Bids:");
        for bid in bid.iter().take(5) {
            let price = bid[0].as_f64().unwrap_or(0.0);
            let size = bid[1].as_i64().unwrap_or(0);
            println!("  Price: {}, Size: {}", &price, &size);
            bids.push((price, size));
        }
    }

    if let Some(ask) = json_data["data"]["asks"].as_array() {
        println!("Top 5 Asks:");
        for ask in ask.iter().take(5) {
            let price = ask[0].as_f64().unwrap_or(0.0);
            let size = ask[1].as_i64().unwrap_or(0);
            println!("  Price: {}, Size: {}", &price, &size);
            asks.push((price, size));
        }
    }

    Ok(())
}
