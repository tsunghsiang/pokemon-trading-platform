use crate::data_type::Card;
use std::collections::HashMap;
use std::collections::LinkedList;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Tag {
    uuid: Uuid,
    id: i32,
}

impl Tag {
    fn new(id: i32, uuid: Uuid) -> Self {
        Self { id, uuid }
    }
}

#[derive(Debug, Clone)]
struct Volume {
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
}

#[derive(Debug, Clone)]
pub struct TxBoard {
    tx_board: HashMap<Card, CardBoard>,
}

impl TxBoard {
    pub fn new() -> Self {
        let mut board = HashMap::new();
        board.insert(Card::Pikachu, CardBoard::new());
        board.insert(Card::Bulbasaur, CardBoard::new());
        board.insert(Card::Charmander, CardBoard::new());
        board.insert(Card::Squirtle, CardBoard::new());
        Self { tx_board: board }
    }
}
