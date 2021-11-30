use crate::data_type::{Card, OrderStatus, Side};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Stats {
    tm: DateTime<Utc>,
    side: Side,
    order_px: f64,
    vol: i32,
    card: Card,
    status: OrderStatus,
}

impl Stats {
    fn new(
        tm: &DateTime<Utc>,
        side: &Side,
        order_px: &f64,
        vol: &i32,
        card: &Card,
        status: &OrderStatus,
    ) -> Self {
        Self {
            tm: tm.clone(),
            side: side.clone(),
            order_px: order_px.clone(),
            vol: vol.clone(),
            card: card.clone(),
            status: status.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBoard {
    // Hash: id -> latest 50 orders
    status_board: HashMap<i32, HashMap<Uuid, Stats>>,
    limit: i32,
}

impl StatusBoard {
    pub fn new() -> Self {
        let board = HashMap::<i32, HashMap<Uuid, Stats>>::new();
        Self {
            status_board: board,
            limit: 50,
        }
    }
}
