use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
struct Volume {
    vol: i32,
    traders: LinkedList<i32>,
}

impl Volume {
    fn new() -> Self {
        Self {
            vol: 0,
            traders: LinkedList::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxBoard {
    // Hash: price -> volume
    buy: HashMap<i32, Volume>,
    sell: HashMap<i32, Volume>,
}

impl TxBoard {
    pub fn new() -> Self {
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
