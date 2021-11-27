use chrono::{DateTime, Utc};
use tide::prelude::{Deserialize, Serialize};

#[derive(Debug)]
enum OrderStatus {
    Ordered,
    Filled,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Card {
    Pikachu,
    Bulbasaur,
    Charmander,
    Squirtle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestOrder {
    tm: DateTime<Utc>,
    side: Side,
    order_px: f64,
    card: Card,
    trader_id: i32,
}
