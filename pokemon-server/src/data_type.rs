use chrono::{DateTime, Utc};
use tide::prelude::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProcessResult {
    TxConfirmed,
    TxFilled,
    TxBoardUpdateFail,
    UnknownCard,
    // add other status here based on real conditions
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Confirmed,
    Filled,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
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

impl RequestOrder {
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_tm(&self) -> DateTime<Utc> {
        self.tm
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_order_px(&self) -> f64 {
        self.order_px
    }

    pub fn get_vol(&self) -> i32 {
        self.vol
    }

    pub fn get_card(&self) -> Card {
        self.card
    }

    pub fn get_trade_id(&self) -> i32 {
        self.trader_id
    }
}
