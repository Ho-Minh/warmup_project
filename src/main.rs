use std::error::Error;
use crate::order_book::OrderBook;
use crate::api::start_websocket_listener;

mod order_book;
mod item;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let mut ob = OrderBook::new();
    
    // Start WebSocket listener for live updates
    start_websocket_listener(&mut ob).await?;

    Ok(())
}
