use crate::data_type::{Card, Side};
use std::collections::HashMap;
use std::collections::LinkedList;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    uuid: Uuid,
    id: i32,
}

impl Tag {
    pub fn new(id: i32, uuid: Uuid) -> Self {
        Self { id, uuid }
    }

    pub fn get_uuid(self) -> Uuid {
        self.uuid
    }

    pub fn get_id(self) -> i32 {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct Volume {
    vol: i32,
    traders: LinkedList<Tag>,
}

impl Volume {
    fn new() -> Self {
        Self {
            vol: 0,
            traders: LinkedList::<Tag>::new(),
        }
    }

    pub fn get_vol(&self) -> &i32 {
        &self.vol
    }

    pub fn set_vol(&mut self, vol: i32) {
        self.vol = vol;
    }

    pub fn pop_trader(&mut self) -> Option<Tag> {
        self.traders.pop_front()
    }

    pub fn push_trader(&mut self, tag: Tag) {
        self.traders.push_back(tag);
    }
}

#[derive(Debug, Clone)]
pub struct CardBoard {
    // Hash: price -> volume
    buy: HashMap<i32, Volume>,
    sell: HashMap<i32, Volume>,
}

impl CardBoard {
    fn new() -> Self {
        let mut buy_map = HashMap::new();
        let mut sell_map = HashMap::new();
        let mut px: i32 = 1;
        while px <= 10 {
            buy_map.insert(px, Volume::new());
            sell_map.insert(px, Volume::new());
            px += 1;
        }
        Self {
            buy: buy_map,
            sell: sell_map,
        }
    }

    pub fn get_bs_board(&mut self, property: Side) -> &mut HashMap<i32, Volume> {
        match property {
            Side::Buy => &mut self.buy,
            Side::Sell => &mut self.sell,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxBoard {
    content: HashMap<Card, CardBoard>,
}

impl TxBoard {
    pub fn new() -> Self {
        let mut board = HashMap::new();
        board.insert(Card::Pikachu, CardBoard::new());
        board.insert(Card::Bulbasaur, CardBoard::new());
        board.insert(Card::Charmander, CardBoard::new());
        board.insert(Card::Squirtle, CardBoard::new());
        Self { content: board }
    }

    pub fn get_board_content(&mut self) -> &mut HashMap<Card, CardBoard> {
        &mut self.content
    }
}
