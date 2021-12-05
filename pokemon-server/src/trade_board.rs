use crate::data_type::Card;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct Trade {
    tx_time: DateTime<Utc>,
    buy_side: i32,
    sell_side: i32,
    tx_price: f64,
    tx_vol: i32,
}

impl Trade {
    pub fn new(
        tx_time: DateTime<Utc>,
        buy_side: i32,
        sell_side: i32,
        tx_price: f64,
        tx_vol: i32,
    ) -> Self {
        Self {
            tx_time,
            buy_side,
            sell_side,
            tx_price,
            tx_vol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradeBoard {
    board: HashMap<Card, LinkedList<Trade>>,
    limit: i32,
}

impl TradeBoard {
    pub fn new() -> Self {
        let mut board = HashMap::new();
        board.insert(Card::Pikachu, LinkedList::<Trade>::new());
        board.insert(Card::Bulbasaur, LinkedList::<Trade>::new());
        board.insert(Card::Charmander, LinkedList::<Trade>::new());
        board.insert(Card::Squirtle, LinkedList::<Trade>::new());
        Self {
            board: board,
            limit: 50,
        }
    }

    pub fn add_trade(&mut self, card: &Card, trade: Trade) {
        if let Some(res) = &mut self.board.get_mut(card) {
            if (res.len() as i32) < self.limit {
                res.push_back(trade);
            } else {
                res.pop_front();
                res.push_back(trade);
            }
        }
    }
}
