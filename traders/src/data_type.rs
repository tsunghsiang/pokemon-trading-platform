use chrono::{DateTime, Utc};
use tide::prelude::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
enum OrderStatus {
    Ordered,
    Filled,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Card {
    Pikachu,
    Bulbasaur,
    Charmander,
    Squirtle,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct RequestOrder {
    uuid: Uuid,
    tm: DateTime<Utc>,
    side: Side,
    order_px: f64,
    vol: i32,
    card: Card,
    trader_id: i32,
}
