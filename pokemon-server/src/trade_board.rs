use crate::data_type::Card;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::collections::LinkedList;
use std::option::Option;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_tx_time(&self) -> &DateTime<Utc> {
        &self.tx_time
    }

    pub fn get_buy_side_id(&self) -> &i32 {
        &self.buy_side
    }

    pub fn get_sell_side_id(&self) -> &i32 {
        &self.sell_side
    }

    pub fn get_tx_price(&self) -> &f64 {
        &self.tx_price
    }

    pub fn get_tx_vol(&self) -> &i32 {
        &self.tx_vol
    }
}

#[derive(Debug, Clone)]
pub struct TradeBoard {
    board: HashMap<Card, LinkedList<Trade>>,
    limit: usize,
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
            if res.len() < self.limit {
                res.push_back(trade);
            } else {
                res.pop_front();
                res.push_back(trade);
            }
        }
    }

    fn get_board_content_mutable(&mut self) -> &mut HashMap<Card, LinkedList<Trade>> {
        &mut self.board
    }

    fn get_board_content_immutable(&self) -> &HashMap<Card, LinkedList<Trade>> {
        &self.board
    }

    fn get_limit(&self) -> &usize {
        &self.limit
    }

    pub fn get_back_trade(&self, card: &Card) -> Option<Trade> {
        let back: Option<Trade> = match self.board.get(card) {
            Some(res) => {
                if let Some(elem) = res.back() {
                    Some(elem.clone())
                } else {
                    Option::None
                }
            }
            None => Option::None,
        };

        back
    }

    pub fn get_front_trade(&self, card: &Card) -> Option<Trade> {
        let front: Option<Trade> = match self.board.get(card) {
            Some(res) => {
                if let Some(elem) = res.front() {
                    Some(elem.clone())
                } else {
                    Option::None
                }
            }
            None => Option::None,
        };

        front
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::Card;
    use crate::trade_board::{Trade, TradeBoard};
    use chrono::Utc;
    use rand::Rng;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn given_a_trade_is_made_when_fields_are_accessed_then_each_should_exists() {
        let (tx_time, buyer, seller, tx_price, tx_vol) = (Utc::now(), 1, 2, 1.00, 1);
        let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
        assert_eq!(&tx_time, trade.get_tx_time());
        assert_eq!(&buyer, trade.get_buy_side_id());
        assert_eq!(&seller, trade.get_sell_side_id());
        assert_eq!(&tx_price, trade.get_tx_price());
        assert_eq!(&tx_vol, trade.get_tx_vol());
    }

    #[test]
    fn given_trade_board_initiated_when_content_accessed_then_each_board_exists() {
        let trade_board = TradeBoard::new();
        assert_eq!(&50, trade_board.get_limit());
        let content = trade_board.get_board_content_immutable();
        if let Some(_) = content.get(&Card::Bulbasaur) {
            // do nothing here, just check if corresponding trade list exists
        } else {
            panic!("[ERROR] Test Failed: Trade List of Bulbasaur does not exist");
        }

        if let Some(_) = content.get(&Card::Charmander) {
            // do nothing here, just check if corresponding trade list exists
        } else {
            panic!("[ERROR] Test Failed: Trade List of Charmander does not exist");
        }

        if let Some(_) = content.get(&Card::Pikachu) {
            // do nothing here, just check if corresponding trade list exists
        } else {
            panic!("[ERROR] Test Failed: Trade List of Pikachu does not exist");
        }

        if let Some(_) = content.get(&Card::Squirtle) {
            // do nothing here, just check if corresponding trade list exists
        } else {
            panic!("[ERROR] Test Failed: Trade List of Squirtle does not exist");
        }

        assert_eq!(4, content.len());
    }

    #[test]
    fn given_limit_has_not_been_reached_when_a_trade_is_made_then_it_is_found_at_tail() {
        // initalize a trade board
        let mut trade_board = TradeBoard::new();
        // initilize a trade component for Bulbasaur
        let (tx_time, buyer, seller, tx_price, tx_vol) = (Utc::now(), 1, 2, 1.00, 1);
        let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
        // add a trade into bulbasaur trade board
        trade_board.add_trade(&Card::Bulbasaur, trade);
        if let Some(res) = trade_board
            .get_board_content_immutable()
            .get(&Card::Bulbasaur)
        {
            if let Some(obj) = res.back() {
                assert_eq!(&tx_time, obj.get_tx_time());
                assert_eq!(&buyer, obj.get_buy_side_id());
                assert_eq!(&seller, obj.get_sell_side_id());
                assert_eq!(&tx_price, obj.get_tx_price());
                assert_eq!(&tx_vol, obj.get_tx_vol());
            } else {
                panic!("[ERROR] Test Failed: Trade of Bulbasaur is Not Inserted");
            }
        } else {
            panic!("[ERROR] Test Failed: Trading List of Bulbasaur does not exist");
        }

        // initilize a trade component for Charmander
        let (tx_time, buyer, seller, tx_price, tx_vol) = (Utc::now(), 2, 3, 4.00, 1);
        let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
        // add a trade into Charmander trade board
        trade_board.add_trade(&Card::Charmander, trade);
        if let Some(res) = trade_board
            .get_board_content_immutable()
            .get(&Card::Charmander)
        {
            if let Some(obj) = res.back() {
                assert_eq!(&tx_time, obj.get_tx_time());
                assert_eq!(&buyer, obj.get_buy_side_id());
                assert_eq!(&seller, obj.get_sell_side_id());
                assert_eq!(&tx_price, obj.get_tx_price());
                assert_eq!(&tx_vol, obj.get_tx_vol());
            } else {
                panic!("[ERROR] Test Failed: Trade of Charmander is Not Inserted");
            }
        } else {
            panic!("[ERROR] Test Failed: Trading List of Charmander does not exist");
        }

        // initilize a trade component for Pikachu
        let (tx_time, buyer, seller, tx_price, tx_vol) = (Utc::now(), 3, 4, 6.00, 1);
        let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
        // add a trade into Pikachu trade board
        trade_board.add_trade(&Card::Pikachu, trade);
        if let Some(res) = trade_board
            .get_board_content_immutable()
            .get(&Card::Pikachu)
        {
            if let Some(obj) = res.back() {
                assert_eq!(&tx_time, obj.get_tx_time());
                assert_eq!(&buyer, obj.get_buy_side_id());
                assert_eq!(&seller, obj.get_sell_side_id());
                assert_eq!(&tx_price, obj.get_tx_price());
                assert_eq!(&tx_vol, obj.get_tx_vol());
            } else {
                panic!("[ERROR] Test Failed: Trade of Pikachu is Not Inserted");
            }
        } else {
            panic!("[ERROR] Test Failed: Trading List of Pikachu does not exist");
        }

        // initilize a trade component for Squirtle
        let (tx_time, buyer, seller, tx_price, tx_vol) = (Utc::now(), 4, 5, 3.00, 1);
        let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
        // add a trade into Squirtle trade board
        trade_board.add_trade(&Card::Squirtle, trade);
        if let Some(res) = trade_board
            .get_board_content_immutable()
            .get(&Card::Squirtle)
        {
            if let Some(obj) = res.back() {
                assert_eq!(&tx_time, obj.get_tx_time());
                assert_eq!(&buyer, obj.get_buy_side_id());
                assert_eq!(&seller, obj.get_sell_side_id());
                assert_eq!(&tx_price, obj.get_tx_price());
                assert_eq!(&tx_vol, obj.get_tx_vol());
            } else {
                panic!("[ERROR] Test Failed: Trade of Squirtle is Not Inserted");
            }
        } else {
            panic!("[ERROR] Test Failed: Trading List of Squirtle does not exist");
        }
    }

    #[test]
    fn given_limit_has_been_reached_when_a_trade_is_made_then_added_into_tail_and_head_is_popped() {
        let trade_board = Rc::new(RefCell::new(TradeBoard::new()));
        let mut rng = rand::thread_rng();
        let (b1, b2, b3, b4, b5) = (
            Rc::clone(&trade_board),
            Rc::clone(&trade_board),
            Rc::clone(&trade_board),
            Rc::clone(&trade_board),
            Rc::clone(&trade_board),
        );
        // fill in maxmum number of trades
        for _ in 1..51 {
            let (tx_time, buyer, seller, tx_price, tx_vol) = (
                Utc::now(),
                rng.gen_range(1..11),
                rng.gen_range(1..11),
                rng.gen_range(1.00..10.00),
                1,
            );
            let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
            // add a trade into bulbasaur trade board
            b1.borrow_mut().add_trade(&Card::Bulbasaur, trade);
        }

        // check if trade list has reached its maxmum limit
        match b2
            .borrow()
            .get_board_content_immutable()
            .get(&Card::Bulbasaur)
        {
            Some(res) => assert_eq!(&50, &res.len()),
            None => panic!("[ERROR] Test Failed: Trade list does not exist"),
        };

        // get the back element first for later comparison

        let back = b3.borrow().get_back_trade(&Card::Bulbasaur);

        // insert a new tradde into trade_board
        let (tx_time, buyer, seller, tx_price, tx_vol) = (
            Utc::now(),
            rng.gen_range(1..11),
            rng.gen_range(1..11),
            rng.gen_range(1.00..10.00),
            1,
        );
        let trade = Trade::new(tx_time, buyer, seller, tx_price, tx_vol);
        b4.borrow_mut().add_trade(&Card::Bulbasaur, trade);
        // compare the old & new back element. they should be different
        let new_back = b5.borrow().get_back_trade(&Card::Bulbasaur);

        assert_ne!(back.unwrap(), new_back.unwrap());
    }
}
