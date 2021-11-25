use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use tide::prelude::{Deserialize, Serialize};

#[derive(Debug)]
enum OrderStatus {
    Ordered,
    Filled,
}

#[derive(Debug, Deserialize)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize)]
enum Card {
    Pikachu,
    Bulbasaur,
    Charmander,
    Squirtle,
}

#[derive(Debug, Deserialize)]
pub struct RequestOrder {
    tm: DateTime<Utc>,
    side: Side,
    order_px: f64,
    card: Card,
    trader_id: i32,
}
