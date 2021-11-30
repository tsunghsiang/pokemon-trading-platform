use chrono::{DateTime, Utc};
use tide::prelude::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProcessResult {
    TxConfirmed,
    TxFilled,
    // add other status here based on real conditions
}

#[derive(Debug, Clone)]
pub enum OrderStatus {
    Confirmed,
    Filled,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
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
