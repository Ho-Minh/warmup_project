use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use crate::order_book::OrderBook;

/// Updates the order book with new bid and ask data from a JSON response.
///
/// This function extracts the top 5 bid and ask levels from the given JSON data,
/// converts them into floating-point price levels and integer sizes, and updates
/// the provided `OrderBook` instance accordingly.
///
/// # Arguments
///
/// * `ob` - A mutable reference to an `OrderBook` instance where the parsed bid and ask data will be stored.
/// * `json_data` - A `serde_json::Value` containing order book data, expected to have `"bids"` and `"asks"` fields.
///
/// # Behavior
///
/// - Extracts up to **5 bid levels** and **5 ask levels** from the `json_data`.
/// - Tries to parse **prices as `f64`** and **sizes as `i64`**, handling cases where values are stored as strings.
/// - Calls `ob.update()` to apply the new bid and ask data.
/// - Calls `ob.print()` to display the updated order book.
///
/// # Example JSON Input
///
/// ```json
/// {
///   "data": {
///     "bids": [["60000.0", "1"], ["59950.0", "2"]],
///     "asks": [["60100.0", "1"], ["60150.0", "3"]]
///   }
/// }
/// ```
///
/// # Example Usage
///
/// ```rust
/// let json_data: serde_json::Value = serde_json::from_str(your_json_string).unwrap();
/// let mut order_book = OrderBook::new();
/// update_order_book(&mut order_book, json_data);
/// ```
///
/// # Notes
///
/// - If a bid or ask **cannot be parsed**, it defaults to `0.0` for price and `0` for size.
/// - This function only **takes the top 5 levels** from the order book update.
///
/// # See Also
///
/// - [`OrderBook::update`] - Method that applies the parsed bid/ask data.
/// - [`OrderBook::print`] - Displays the updated order book.
fn update_order_book(ob: &mut OrderBook, json_data: Value) {
    let mut bids = vec![];
    let mut asks = vec![];

    // Parse initial bids & asks
    if let Some(bid_array) = json_data["data"]["bids"].as_array() {
        for bid in bid_array.iter().take(5) {
            let price = bid[0].as_f64().unwrap_or(bid[0].as_str()
            .and_then(|s| s.parse::<f64>().ok()) // Try parsing it
            .unwrap_or(0.0));
            let size = bid[1].as_i64().unwrap_or(bid[1].as_str()
            .and_then(|s| s.parse::<i64>().ok()) // Try parsing it
            .unwrap_or(0));
            bids.push((price, size));
        }
    }

    if let Some(ask_array) = json_data["data"]["asks"].as_array() {
        for ask in ask_array.iter().take(5) {
            let price = ask[0].as_f64().unwrap_or(ask[0].as_str()
            .and_then(|s| s.parse::<f64>().ok()) // Try parsing it
            .unwrap_or(0.0));
            let size = ask[1].as_i64().unwrap_or(ask[1].as_str()
            .and_then(|s| s.parse::<i64>().ok()) // Try parsing it
            .unwrap_or(0));
            asks.push((price, size));
        }
    }

    ob.update(bids, asks);
    ob.print();
}

/// Establishes a WebSocket connection to the KuCoin Futures API and listens for real-time order book updates.
///
/// This function:
/// - Requests a **WebSocket token** from the KuCoin API.
/// - Extracts the **WebSocket URL** and establishes a **secure connection**.
/// - **Subscribes to order book updates** for `ETHUSDTM`.
/// - **Processes and applies market updates** to the provided `OrderBook`.
///
/// # Arguments
///
/// * `ob` - A mutable reference to an `OrderBook` instance that will be updated in real time.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error wrapped in `Box<dyn Error>` if any failure occurs.
///
/// # Behavior
///
/// - Fetches a **temporary WebSocket token** from `https://api-futures.kucoin.com/api/v1/bullet-public`.
/// - Connects to the **KuCoin Futures WebSocket endpoint** (`wss://ws-api-futures.kucoin.com/`).
/// - Sends a subscription request for the **top 5 levels** of the ETHUSDTM order book (`/contractMarket/level2Depth5:ETHUSDTM`).
/// - Listens for **real-time bid/ask updates** and updates the `OrderBook` accordingly.
///
/// # Example Usage
///
/// ```rust
/// let mut order_book = OrderBook::new();
/// start_websocket_listener(&mut order_book).await.unwrap();
/// ```
///
/// # Notes
///
/// - This function **runs indefinitely** and should be executed in an async runtime.
/// - If the **WebSocket connection is lost**, the function **exits**, and you may need to restart it.
/// - WebSocket tokens are **short-lived**, so reconnecting requires requesting a new token.
///
/// # See Also
///
/// - [`update_order_book`] - Processes order book updates received via WebSocket.
pub async fn start_websocket_listener(ob: &mut OrderBook) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let ws_token_url = "https://api-futures.kucoin.com/api/v1/bullet-public";

    // 1️⃣ Fetch WebSocket token
    let response = client.post(ws_token_url).send().await?;
    let response_text = response.text().await?;
    let json_data: Value = serde_json::from_str(&response_text)?;

    // 2️⃣ Extract WebSocket URL & Token
    let ws_url = json_data["data"]["instanceServers"][0]["endpoint"]
        .as_str()
        .ok_or("WebSocket URL not found")?;
    
    let token = json_data["data"]["token"]
        .as_str()
        .ok_or("WebSocket Token not found")?;

    let full_ws_url = format!("{}?token={}", ws_url, token); // ✅ Include token in WebSocket URL
    
    println!("Connecting to WebSocket: {}", full_ws_url);

    // 3️⃣ Connect to KuCoin WebSocket
    let (ws_stream, _) = connect_async(full_ws_url).await.expect("Failed to connect to WebSocket");
    println!("✅ Connected to KuCoin WebSocket");

    let (mut write, mut read) = ws_stream.split();

    // 4️⃣ Subscribe to order book updates
    let subscription_msg = serde_json::json!({
        "id": "1",
        "type": "subscribe",
        "topic": "/contractMarket/level2Depth5:ETHUSDTM",
        "response": true
    })
    .to_string();

    write.send(Message::Text(subscription_msg)).await.expect("Failed to send subscription message");

    // ✅ Confirm subscription response
    if let Some(msg) = read.next().await {
        if let Ok(Message::Text(text)) = msg {
            println!("🔹 Subscription Response: {}", text);
        }
    }

    // 5️⃣ Listen for updates
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("📩 WebSocket Message: {}", text); // ✅ Debugging Output
                
                if let Ok(json_data) = serde_json::from_str::<Value>(&text) {
                    if json_data["type"] == "message" {
                        update_order_book(ob, json_data);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                eprintln!("❌ WebSocket Closed by Server.");
                break;
            }
            Err(err) => {
                eprintln!("❌ WebSocket Error: {}", err);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
