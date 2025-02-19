use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Item {
    pub price: f64,
    pub size: i64,
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.total_cmp(&other.price)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
