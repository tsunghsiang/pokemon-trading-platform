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
}
