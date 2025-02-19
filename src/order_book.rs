use std::collections::BTreeSet;
use prettytable::{Table, Row, Cell};
use crate::item::Item;

pub struct OrderBook {
    bids: BTreeSet<Item>,
    asks: BTreeSet<Item>,
}

impl OrderBook {
    
    /// Creates a new, empty `OrderBook`.
    ///
    /// # Examples
    ///
    /// ```
    /// let order_book = OrderBook::new();
    /// ```
    pub fn new() -> Self {
        Self {
            bids: BTreeSet::new(),
            asks: BTreeSet::new(),
        }
    }

    /// Updates the order book with new bid and ask data.
    ///
    /// If the size of either the bids or asks set exceeds 5, pop to maintain the sizec
    ///
    /// # Arguments
    ///
    /// * `bids` - A vector of tuples representing bid orders as (price, size).
    /// * `asks` - A vector of tuples representing ask orders as (price, size).
    ///
    /// # Examples
    ///
    /// ```
    /// // Update with new bids and asks.
    /// order_book.update(vec![(2000.0, 15)], vec![(2010.0, 20)]);
    /// ```
    pub fn update(&mut self, bids: Vec<(f64, i64)>, asks: Vec<(f64, i64)>) {
        self.bids.clear();
        self.asks.clear();
        for item in bids {
            self.bids.insert(Item {price: item.0, size: item.1});
        }

        for item in asks {
            self.asks.insert(Item {price: item.0, size: item.1});
        }
    }

    /// Prints the current state of the order book in a columnar format.
    ///
    /// Displays the top 5 bids (highest prices) and top 5 asks (lowest prices).
    ///
    /// # Examples
    ///
    /// ```
    /// order_book.print();
    /// ```
    pub fn print(&self) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Type"),
            Cell::new("Symbol"),
            Cell::new("Price"),
            Cell::new("Contract size"),
        ]));

        for item in &self.bids {
            table.add_row(Row::new(vec![
                Cell::new("Bids"),
                Cell::new("ETHUSDTM"),
                Cell::new(&item.price.to_string()),
                Cell::new(&item.size.to_string()),
            ]));
        }

        for item in self.asks.iter().rev() {
            table.add_row(Row::new(vec![
                Cell::new("Asks"),
                Cell::new("ETHUSDTM"),
                Cell::new(&item.price.to_string()),
                Cell::new(&item.size.to_string()),
            ]));
        }
        print!("Current order book state\n");
        table.printstd();
    }

}