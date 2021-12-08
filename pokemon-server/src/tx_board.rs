use crate::data_type::{Card, Side};
use std::collections::HashMap;
use std::collections::LinkedList;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_trader_nums(&self) -> usize {
        self.traders.len()
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

#[cfg(test)]
mod tests {
    use crate::data_type::{Card, Side};
    use crate::tx_board::{CardBoard, Tag, TxBoard, Volume};
    use std::sync::Arc;
    use std::sync::Mutex;
    use uuid::Uuid;

    #[test]
    fn given_uuid_and_id_when_tag_instatiated_then_could_read_fields() {
        let (id, uuid) = (1, Uuid::new_v4());
        let tag: Tag = Tag::new(id, uuid);
        assert_eq!(id, tag.clone().get_id());
        assert_eq!(uuid, tag.clone().get_uuid());
    }

    #[test]
    fn given_volume_initiated_then_vol_is_zero_and_none_popped_from_traders_field() {
        let obj: Volume = Volume::new();
        assert_eq!(&0, obj.clone().get_vol());
        assert_eq!(None, obj.clone().pop_trader());
    }

    #[test]
    fn given_volume_configured_with_vol_and_tags_when_fields_accessed_then_all_are_verifiable() {
        let tag1: Tag = Tag::new(1, Uuid::new_v4());
        let tag2: Tag = Tag::new(2, Uuid::new_v4());
        let mut obj: Volume = Volume::new();
        obj.set_vol(2);
        assert_eq!(&2, obj.get_vol());
        obj.push_trader(tag1);
        obj.push_trader(tag2);
        assert_eq!(2, obj.get_trader_nums());
    }

    #[test]
    fn given_cardboard_initiated_when_volume_accessed_by_key_then_field_vol_is_zero() {
        let board = Arc::new(Mutex::new(CardBoard::new()));
        let (buy_board, sell_board) = (Arc::clone(&board), Arc::clone(&board));
        for px in 1..11 {
            if let Ok(mut res) = buy_board.lock() {
                let card_buy_board = res.get_bs_board(Side::Buy);
                if let Some(obj) = card_buy_board.get(&px) {
                    assert_eq!(&0, obj.get_vol());
                } else {
                    panic!("[ERROR] Test Failed: Cannot Access Corresponding Buy Volume");
                }
            } else {
                panic!("[ERROR] Test Failed When Accessing Card Buy Board");
            }

            if let Ok(mut res) = sell_board.lock() {
                let card_sell_board = res.get_bs_board(Side::Buy);
                if let Some(obj) = card_sell_board.get(&px) {
                    assert_eq!(&0, obj.get_vol());
                } else {
                    panic!("[ERROR] Test Failed: Cannot Access Corresponding Sell Volume");
                }
            } else {
                panic!("[ERROR] Test Failed When Accessing Card Buy Board");
            }
        }
    }

    #[test]
    fn given_txboard_initiated_when_accessed_by_card_then_corresponding_cardboards_exist() {
        let mut tx_board = TxBoard::new();
        let content = tx_board.get_board_content();
        if let Some(_cardboard) = content.get(&Card::Bulbasaur) {
            // do nothing here, just check corresponding card board exists
        } else {
            panic!("[ERROR] TxBoard instantiation error: Bulbasaur does not exist.");
        }

        if let Some(_cardboard) = content.get(&Card::Charmander) {
            // do nothing here, just check corresponding card board exists
        } else {
            panic!("[ERROR] TxBoard instantiation error: Charmander does not exist.");
        }

        if let Some(_cardboard) = content.get(&Card::Pikachu) {
            // do nothing here, just check corresponding card board exists
        } else {
            panic!("[ERROR] TxBoard instantiation error: Pikachu does not exist.");
        }

        if let Some(_cardboard) = content.get(&Card::Squirtle) {
            // do nothing here, just check corresponding card board exists
        } else {
            panic!("[ERROR] TxBoard instantiation error: Squirtle does not exist.");
        }

        assert_eq!(4, content.len());
    }
}
