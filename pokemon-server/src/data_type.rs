use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use tide::prelude::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessResult {
    TxConfirmed,
    TxFilled,
    TxBoardUpdateFail,
    TxSelfTraded,
    UnknownCard,
    // add other status here based on real conditions
}

#[derive(Debug, ToSql, FromSql, Clone, PartialEq)]
#[postgres(name = "orderstatus")]
pub enum OrderStatus {
    #[postgres(name = "Confirmed")]
    Confirmed,
    #[postgres(name = "Filled")]
    Filled,
    #[postgres(name = "Dropped")]
    Dropped,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, ToSql, FromSql)]
#[postgres(name = "side")]
pub enum Side {
    #[postgres(name = "Buy")]
    Buy,
    #[postgres(name = "Sell")]
    Sell,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, ToSql, FromSql)]
#[postgres(name = "card")]
pub enum Card {
    #[postgres(name = "Pikachu")]
    Pikachu,
    #[postgres(name = "Bulbasaur")]
    Bulbasaur,
    #[postgres(name = "Charmander")]
    Charmander,
    #[postgres(name = "Squirtle")]
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
    pub fn new(
        uuid: Uuid,
        tm: DateTime<Utc>,
        side: Side,
        order_px: f64,
        vol: i32,
        card: Card,
        trade_id: i32,
    ) -> RequestOrder {
        Self {
            uuid: uuid,
            tm: tm,
            side: side,
            order_px: order_px,
            vol: vol,
            card: card,
            trader_id: trade_id,
        }
    }

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

    pub fn to_str(&self) -> String {
        let mut res = String::from("");
        let fmt = format!(" uuid: {}, tm: {}, side: {:?}, order_px: {}, vol: {}, card: {:?}, trade_id: {} ", self.uuid, self.tm, self.side, self.order_px, self.vol, self.card, self.trader_id);
        res.push('{');
        res.push_str(&fmt);
        res.push('}');
        res
    }
}
