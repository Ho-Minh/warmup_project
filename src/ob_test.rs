///Local testing
///This tests the parsing and the OrderBook

use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::order_book::OrderBook;  // Import everything from `order_book`
use crate::item::Item;

pub fn ob_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("Path");
    let path = Path::new("src/test.txt");

    println!("Contents");
    // Read the file contents into a String
    let contents = fs::read_to_string(path)?;

    let json_data: Value = serde_json::from_str(&contents)?;

    println!("json_data");

    let mut bids: Vec<(f64, i64)> = vec![];
    let mut asks: Vec<(f64, i64)> = vec![];


    // Parse bids & asks manually (for performance)
    if let Some(bid) = json_data["data"]["bids"].as_array() {
        println!("Top 5 Bids:");
        for bid in bid.iter().take(5) {
            let price = bid[0].as_f64().unwrap_or(bid[0].as_str()
            .and_then(|s| s.parse::<f64>().ok()) // Try parsing it
            .unwrap_or(0.0));
            let size = bid[1].as_i64().unwrap_or(bid[1].as_str()
            .and_then(|s| s.parse::<i64>().ok()) // Try parsing it
            .unwrap_or(0));
            println!("  Price: {}, Size: {}", &price, &size);
            bids.push((price, size));
        }
    }

    if let Some(ask) = json_data["data"]["asks"].as_array() {
        println!("Top 5 Asks:");
        for ask in ask.iter().take(5) {
            let price = ask[0].as_f64().unwrap_or(ask[0].as_str()
            .and_then(|s| s.parse::<f64>().ok()) // Try parsing it
            .unwrap_or(0.0));
            let size = ask[1].as_i64().unwrap_or(ask[1].as_str()
            .and_then(|s| s.parse::<i64>().ok()) // Try parsing it
            .unwrap_or(0));
            println!("  Price: {}, Size: {}", &price, &size);
            asks.push((price, size));
        }
    }

    let mut ob = OrderBook::new();
    ob.update(bids, asks);
    ob.print();

    Ok(())
}
