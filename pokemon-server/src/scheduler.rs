use crate::data_type::{Card, OrderStatus, ProcessResult, RequestOrder, Side};
use crate::database;
use crate::status_board::{Stats, StatusBoard};
use crate::trade_board::{Trade, TradeBoard};
use crate::tx_board::{Tag, TxBoard};

use chrono::Utc;
use database::Database;
use std::collections::{LinkedList, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use tide::Request;
use uuid::Uuid;

pub struct Scheduler {
    pub order_queue: VecDeque<RequestOrder>,
    pub tx_board: TxBoard,
    pub trade_board: TradeBoard,
    pub status_board: StatusBoard,
    pub db: Database,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            order_queue: VecDeque::<RequestOrder>::new(),
            tx_board: TxBoard::new(),
            trade_board: TradeBoard::new(),
            status_board: StatusBoard::new(),
            db: Database::new(),
        }
    }

    pub fn process(&mut self, req: &RequestOrder) -> ProcessResult {
        let mut proc_res: ProcessResult = ProcessResult::TxConfirmed;
        let card = req.get_card();
        self.db.insert_request_table(req);

        if let Some(res) = self.tx_board.get_board_content().get_mut(&card) {
            proc_res = match req.get_side() {
                Side::Buy => {
                    let mut px = 1;
                    let sell_card_board = res.get_bs_board(Side::Sell);
                    loop {
                        if let Some(volume) = sell_card_board.get_mut(&px) {
                            // Buy order traded
                            if volume.get_vol() > &0 && req.get_order_px() >= (px as f64) {
                                // update tx_board
                                let mut uuid: Uuid;
                                let mut sell_side_id: i32 = -1;
                                volume.set_vol(volume.get_vol() - 1);
                                if let Some(tag) = volume.pop_trader() {
                                    uuid = tag.clone().get_uuid();
                                    sell_side_id = tag.clone().get_id();
                                } else {
                                    break ProcessResult::TxBoardUpdateFail;
                                }

                                // update trade_board
                                let trade = Trade::new(
                                    Utc::now(),
                                    req.get_trade_id(),
                                    sell_side_id,
                                    px as f64,
                                    req.get_vol(),
                                );

                                self.trade_board.add_trade(&req.get_card(), trade);

                                // update status board
                                // update sell-side's status_board (update)
                                self.status_board.update_status(
                                    sell_side_id,
                                    uuid,
                                    OrderStatus::Filled,
                                );

                                // update buy-side's status board (add)
                                let stats = Stats::new(
                                    req.get_uuid(),
                                    Utc::now(),
                                    Side::Buy,
                                    req.get_order_px(),
                                    req.get_vol(),
                                    req.get_card(),
                                    OrderStatus::Filled,
                                );
                                self.status_board.add_status(
                                    req.get_trade_id(),
                                    req.get_uuid(),
                                    stats,
                                );
                                println!(
                                    "[BUY][FILLED] Card: {:?}, TxPrice: {}, TxVol: {}",
                                    &card,
                                    &px,
                                    req.get_vol()
                                );
                                break ProcessResult::TxFilled;
                            } else {
                                px += 1;
                            }
                        }

                        if px > 10 {
                            // update tx_board
                            let buy_card_board = res.get_bs_board(Side::Buy);
                            let tag = Tag::new(req.get_trade_id(), req.get_uuid());
                            if let Some(cur_vol) =
                                buy_card_board.get_mut(&(req.get_order_px() as i32))
                            {
                                cur_vol.set_vol(cur_vol.get_vol() + req.get_vol());
                                cur_vol.push_trader(tag);
                            }

                            // update status baord
                            let stats = Stats::new(
                                req.get_uuid(),
                                req.get_tm(),
                                req.get_side(),
                                req.get_order_px(),
                                req.get_vol(),
                                req.get_card(),
                                OrderStatus::Confirmed,
                            );
                            self.status_board
                                .add_status(req.get_trade_id(), req.get_uuid(), stats);
                            println!(
                                "[BUY][CONFIRMED] Card: {:?}, OrderPx: {}, Volume: {}, TradeId: {}",
                                req.get_card(),
                                req.get_order_px(),
                                req.get_vol(),
                                req.get_trade_id()
                            );
                            break ProcessResult::TxConfirmed;
                        }
                    }
                }
                Side::Sell => {
                    let mut px = 10;
                    let buy_card_board = res.get_bs_board(Side::Buy);
                    loop {
                        if let Some(volume) = buy_card_board.get_mut(&px) {
                            // Sell order traded
                            if volume.get_vol() > &0 && req.get_order_px() <= (px as f64) {
                                // update tx_board
                                let mut uuid: Uuid;
                                let mut buy_side_id: i32 = -1;
                                volume.set_vol(volume.get_vol() - 1);
                                if let Some(tag) = volume.pop_trader() {
                                    uuid = tag.clone().get_uuid();
                                    buy_side_id = tag.clone().get_id();
                                } else {
                                    break ProcessResult::TxBoardUpdateFail;
                                }
                                // update trade_board
                                let trade = Trade::new(
                                    Utc::now(),
                                    buy_side_id,
                                    req.get_trade_id(),
                                    px as f64,
                                    req.get_vol(),
                                );
                                self.trade_board.add_trade(&req.get_card(), trade);
                                // update status board
                                // update buy-side's status_board (update)
                                self.status_board.update_status(
                                    buy_side_id,
                                    uuid,
                                    OrderStatus::Filled,
                                );
                                // update sell-side's status board (add)
                                let stats = Stats::new(
                                    req.get_uuid(),
                                    Utc::now(),
                                    Side::Sell,
                                    req.get_order_px(),
                                    req.get_vol(),
                                    req.get_card(),
                                    OrderStatus::Filled,
                                );
                                self.status_board.add_status(
                                    req.get_trade_id(),
                                    req.get_uuid(),
                                    stats,
                                );
                                println!(
                                    "[SELL][FILLED] Card: {:?}, TxPrice: {}, TxVol: {}",
                                    &card,
                                    &px,
                                    req.get_vol()
                                );
                                break ProcessResult::TxFilled;
                            } else {
                                px -= 1;
                            }
                        }
                        if px < 1 {
                            // update tx_board
                            let sell_card_board = res.get_bs_board(Side::Sell);
                            let tag = Tag::new(req.get_trade_id(), req.get_uuid());
                            if let Some(cur_vol) =
                                sell_card_board.get_mut(&(req.get_order_px() as i32))
                            {
                                cur_vol.set_vol(cur_vol.get_vol() + req.get_vol());
                                cur_vol.push_trader(tag);
                            }

                            // update status baord
                            let stats = Stats::new(
                                req.get_uuid(),
                                req.get_tm(),
                                req.get_side(),
                                req.get_order_px(),
                                req.get_vol(),
                                req.get_card(),
                                OrderStatus::Confirmed,
                            );
                            self.status_board
                                .add_status(req.get_trade_id(), req.get_uuid(), stats);
                            println!(
                                "[SELL][CONFIRMED] Card: {:?}, OrderPx: {}, Volume: {}, TradeId: {}",
                                req.get_card(),
                                req.get_order_px(),
                                req.get_vol(),
                                req.get_trade_id()
                            );
                            break ProcessResult::TxConfirmed;
                        }
                    }
                }
            };
        } else {
            proc_res = ProcessResult::UnknownCard;
        }
        proc_res
    }

    pub fn get_latest_trades(&self, card: &Card) -> Option<&LinkedList<Trade>> {
        self.trade_board.get_board_content_immutable().get(card)
    }

    pub fn get_latest_orders(&self, id: &i32) -> Option<LinkedList<Stats>> {
        let mut res: LinkedList<Stats> = LinkedList::<Stats>::new();
        if let Some(uuids) = self.status_board.get_latest_uuids(id) {
            for e in uuids {
                if let Some(stat) = self.status_board.get_stat(id, &e) {
                    res.push_back(stat);
                }
            }
            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{Card, OrderStatus, ProcessResult, RequestOrder, Side};
    use crate::status_board::Stats;
    use crate::trade_board::Trade;
    use crate::Scheduler;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn given_there_are_not_sell_orders_when_a_buy_order_received_then_confirmed() {
        let mut scheduler = Scheduler::new();
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            5.00,
            1,
            Card::Bulbasaur,
            1,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        assert_eq!(ProcessResult::TxConfirmed, scheduler.process(&req));
    }
    #[test]
    fn given_there_are_not_sell_orders_when_a_buy_order_received_then_queued_in_tx_board() {
        let mut scheduler = Scheduler::new();
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            5.00,
            1,
            Card::Bulbasaur,
            1,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into tx_board for later matching
        if let Some(card_board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&req.get_card())
        {
            if let Some(volume) = card_board
                .get_bs_board(req.get_side())
                .get_mut(&(req.get_order_px() as i32))
            {
                assert_eq!(true, volume.get_vol() > &0);
                assert_eq!(1, volume.get_trader_nums());
                if let Some(tag) = volume.pop_trader() {
                    assert_eq!(req.get_uuid(), tag.clone().get_uuid());
                    assert_eq!(req.get_trade_id(), tag.clone().get_id())
                } else {
                    panic!("[ERROR] Test Failed: Tag does not exist.");
                }
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist in card board.");
            }
        } else {
            panic!("[ERROR] Test Failed: Card board does not exist.");
        }
    }
    #[test]
    fn given_there_are_not_sell_orders_when_a_buy_order_received_then_queue_in_status_board() {
        let mut scheduler = Scheduler::new();
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            5.00,
            1,
            Card::Bulbasaur,
            1,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into status board
        if let Some(res) = scheduler.status_board.get_back_uuid(&req.get_trade_id()) {
            assert_eq!(req.get_uuid(), res);
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler
            .status_board
            .get_stat(&req.get_trade_id(), &req.get_uuid())
        {
            assert_eq!(&req.get_card(), res.get_card());
            assert_eq!(&req.get_order_px(), res.get_order_px());
            assert_eq!(&req.get_side(), res.get_side());
            assert_eq!(&OrderStatus::Confirmed, res.get_status());
            assert_eq!(&req.get_tm(), res.get_tm());
            assert_eq!(&req.get_uuid(), res.get_uuid());
            assert_eq!(&req.get_vol(), res.get_vol());
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_board");
        }
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_an_untradable_price_received_then_confirmed(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            5.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        assert_eq!(ProcessResult::TxConfirmed, scheduler.process(&req));
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_an_untradable_price_received_then_queued_in_tx_board(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            5.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into tx_board for later matching
        if let Some(card_board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&req.get_card())
        {
            if let Some(volume) = card_board
                .get_bs_board(req.get_side())
                .get_mut(&(req.get_order_px() as i32))
            {
                assert_eq!(true, volume.get_vol() > &0);
                assert_eq!(1, volume.get_trader_nums());
                if let Some(tag) = volume.pop_trader() {
                    assert_eq!(req.get_uuid(), tag.clone().get_uuid());
                    assert_eq!(req.get_trade_id(), tag.clone().get_id())
                } else {
                    panic!("[ERROR] Test Failed: Tag does not exist.");
                }
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist in card board.");
            }
        } else {
            panic!("[ERROR] Test Failed: Card board does not exist.");
        }
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_an_untradable_price_received_then_queued_in_status_board(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            5.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into status board
        if let Some(res) = scheduler.status_board.get_back_uuid(&req.get_trade_id()) {
            assert_eq!(req.get_uuid(), res);
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler
            .status_board
            .get_stat(&req.get_trade_id(), &req.get_uuid())
        {
            assert_eq!(&req.get_card(), res.get_card());
            assert_eq!(&req.get_order_px(), res.get_order_px());
            assert_eq!(&req.get_side(), res.get_side());
            assert_eq!(&OrderStatus::Confirmed, res.get_status());
            assert_eq!(&req.get_tm(), res.get_tm());
            assert_eq!(&req.get_uuid(), res.get_uuid());
            assert_eq!(&req.get_vol(), res.get_vol());
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_board");
        }
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_a_tradable_price_received_then_filled() {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            8.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // process request
        assert_eq!(ProcessResult::TxFilled, scheduler.process(&req));
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_a_tradable_price_received_then_tx_board_updated(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            8.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // get field state of the lowest sell price
        let prev_vol: i32;
        let prev_trader_cnt: usize;
        if let Some(board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&Card::Bulbasaur)
        {
            if let Some(volume) = board.get_bs_board(Side::Sell).get_mut(&6) {
                prev_vol = *volume.get_vol();
                prev_trader_cnt = volume.get_trader_nums();
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist.");
            }
        } else {
            panic!("[ERROR] Test Failed: Board does not exist.");
        }

        // process request
        scheduler.process(&req);

        // get field state of the lowest sell price
        let cur_vol: i32;
        let cur_trader_cnt: usize;
        if let Some(board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&Card::Bulbasaur)
        {
            if let Some(volume) = board.get_bs_board(Side::Sell).get_mut(&6) {
                cur_vol = *volume.get_vol();
                cur_trader_cnt = volume.get_trader_nums();
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist.");
            }
        } else {
            panic!("[ERROR] Test Failed: Board does not exist.");
        }

        assert!(cur_vol < prev_vol);
        assert!(cur_trader_cnt < prev_trader_cnt);
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_a_tradable_price_received_then_trade_board_updated(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            8.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // get previous state of trade_board
        let prev_trade: Trade;
        let cur_trade: Trade;
        let tm = Utc::now();
        if let Some(res) = scheduler.trade_board.get_back_trade(&Card::Bulbasaur) {
            prev_trade = res.clone();
        } else {
            prev_trade = Trade::new(tm.clone(), 1, 1, 1.00, 1);
        }

        // process the request
        scheduler.process(&req);

        // get current state of trade_board

        if let Some(res) = scheduler.trade_board.get_back_trade(&Card::Bulbasaur) {
            cur_trade = res.clone();
        } else {
            cur_trade = Trade::new(tm.clone(), 1, 1, 1.00, 1);
        }

        assert_ne!(cur_trade, prev_trade);
        assert_eq!(&6, cur_trade.get_buy_side_id());
        assert_eq!(&5, cur_trade.get_sell_side_id());
        assert_eq!(&6.00, cur_trade.get_tx_price());
        assert_eq!(&1, cur_trade.get_tx_vol());
    }

    #[test]
    fn given_there_are_sell_orders_when_a_buy_order_with_a_tradable_price_received_then_status_board_updated(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Sell,
                (11.00 - i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate buy order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            8.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // get previous state of status board
        let prev_sell_side_uuid: Uuid;
        let prev_sell_side_stat: Stats;
        if let Some(res) = scheduler.status_board.get_back_uuid(&5) {
            prev_sell_side_uuid = res.clone();
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler.status_board.get_stat(&5, &prev_sell_side_uuid) {
            prev_sell_side_stat = res.clone();
            assert_eq!(&OrderStatus::Confirmed, prev_sell_side_stat.get_status());
        } else {
            panic!("[ERROR] Test Failed: stat does not exist in status_board");
        }

        // process order request
        scheduler.process(&req);

        // get current state of status board
        let cur_sell_side_stat: Stats;
        let cur_buy_side_stat: Stats;
        let cur_buy_side_uuid: Uuid;
        if let Some(res) = scheduler.status_board.get_stat(&5, &prev_sell_side_uuid) {
            cur_sell_side_stat = res.clone();
            assert_eq!(&OrderStatus::Filled, cur_sell_side_stat.get_status());
        } else {
            panic!("[ERROR] Test Failed: stat does not exist in status_board");
        }

        if let Some(res) = scheduler.status_board.get_back_uuid(&6) {
            cur_buy_side_uuid = res.clone();
            assert_eq!(
                Some(cur_buy_side_uuid),
                scheduler.status_board.get_back_uuid(&6)
            );
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler.status_board.get_stat(&6, &cur_buy_side_uuid) {
            cur_buy_side_stat = res.clone();
            assert_eq!(&OrderStatus::Filled, cur_buy_side_stat.get_status());
        } else {
            panic!("[ERROR] Test Failed: stat does not exist in status_board");
        }
    }

    #[test]
    fn given_there_are_not_buy_orders_when_a_sell_order_received_then_confirmed() {
        let mut scheduler = Scheduler::new();
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            5.00,
            1,
            Card::Bulbasaur,
            1,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        assert_eq!(ProcessResult::TxConfirmed, scheduler.process(&req));
    }

    #[test]
    fn given_there_are_not_buy_orders_when_a_sell_order_received_then_queued_in_tx_board() {
        let mut scheduler = Scheduler::new();
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            5.00,
            1,
            Card::Bulbasaur,
            1,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into tx_board for later matching
        if let Some(card_board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&req.get_card())
        {
            if let Some(volume) = card_board
                .get_bs_board(req.get_side())
                .get_mut(&(req.get_order_px() as i32))
            {
                assert_eq!(true, volume.get_vol() > &0);
                assert_eq!(1, volume.get_trader_nums());
                if let Some(tag) = volume.pop_trader() {
                    assert_eq!(req.get_uuid(), tag.clone().get_uuid());
                    assert_eq!(req.get_trade_id(), tag.clone().get_id())
                } else {
                    panic!("[ERROR] Test Failed: Tag does not exist.");
                }
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist in card board.");
            }
        } else {
            panic!("[ERROR] Test Failed: Card board does not exist.");
        }
    }

    #[test]
    fn given_there_are_not_buy_orders_when_a_sell_order_received_then_queue_in_status_board() {
        let mut scheduler = Scheduler::new();
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            5.00,
            1,
            Card::Bulbasaur,
            1,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into status board
        if let Some(res) = scheduler.status_board.get_back_uuid(&req.get_trade_id()) {
            assert_eq!(req.get_uuid(), res);
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler
            .status_board
            .get_stat(&req.get_trade_id(), &req.get_uuid())
        {
            assert_eq!(&req.get_card(), res.get_card());
            assert_eq!(&req.get_order_px(), res.get_order_px());
            assert_eq!(&req.get_side(), res.get_side());
            assert_eq!(&OrderStatus::Confirmed, res.get_status());
            assert_eq!(&req.get_tm(), res.get_tm());
            assert_eq!(&req.get_uuid(), res.get_uuid());
            assert_eq!(&req.get_vol(), res.get_vol());
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_board");
        }
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_an_untradable_price_received_then_confirmed(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            6.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        assert_eq!(ProcessResult::TxConfirmed, scheduler.process(&req));
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_an_untradable_price_received_then_queued_in_tx_board(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            6.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into tx_board for later matching
        if let Some(card_board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&req.get_card())
        {
            if let Some(volume) = card_board
                .get_bs_board(req.get_side())
                .get_mut(&(req.get_order_px() as i32))
            {
                assert_eq!(true, volume.get_vol() > &0);
                assert_eq!(1, volume.get_trader_nums());
                if let Some(tag) = volume.pop_trader() {
                    assert_eq!(req.get_uuid(), tag.clone().get_uuid());
                    assert_eq!(req.get_trade_id(), tag.clone().get_id())
                } else {
                    panic!("[ERROR] Test Failed: Tag does not exist.");
                }
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist in card board.");
            }
        } else {
            panic!("[ERROR] Test Failed: Card board does not exist.");
        }
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_an_untradable_price_received_then_queued_in_status_board(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            6.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // process request
        scheduler.process(&req);
        // check if non-filled order is inserted into status board
        if let Some(res) = scheduler.status_board.get_back_uuid(&req.get_trade_id()) {
            assert_eq!(req.get_uuid(), res);
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler
            .status_board
            .get_stat(&req.get_trade_id(), &req.get_uuid())
        {
            assert_eq!(&req.get_card(), res.get_card());
            assert_eq!(&req.get_order_px(), res.get_order_px());
            assert_eq!(&req.get_side(), res.get_side());
            assert_eq!(&OrderStatus::Confirmed, res.get_status());
            assert_eq!(&req.get_tm(), res.get_tm());
            assert_eq!(&req.get_uuid(), res.get_uuid());
            assert_eq!(&req.get_vol(), res.get_vol());
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_board");
        }
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_a_tradable_price_received_then_filled() {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            4.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // process request
        assert_eq!(ProcessResult::TxFilled, scheduler.process(&req));
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_a_tradable_price_received_then_tx_board_updated(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            4.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // get field state of the highest buy price
        let prev_vol: i32;
        let prev_trader_cnt: usize;
        if let Some(board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&Card::Bulbasaur)
        {
            if let Some(volume) = board.get_bs_board(Side::Buy).get_mut(&5) {
                prev_vol = *volume.get_vol();
                prev_trader_cnt = volume.get_trader_nums();
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist.");
            }
        } else {
            panic!("[ERROR] Test Failed: Board does not exist.");
        }

        // process request
        scheduler.process(&req);

        // get field state of the highest buy price
        let cur_vol: i32;
        let cur_trader_cnt: usize;
        if let Some(board) = scheduler
            .tx_board
            .get_board_content()
            .get_mut(&Card::Bulbasaur)
        {
            if let Some(volume) = board.get_bs_board(Side::Buy).get_mut(&5) {
                cur_vol = *volume.get_vol();
                cur_trader_cnt = volume.get_trader_nums();
            } else {
                panic!("[ERROR] Test Failed: Volume does not exist.");
            }
        } else {
            panic!("[ERROR] Test Failed: Board does not exist.");
        }

        assert!(cur_vol < prev_vol);
        assert!(cur_trader_cnt < prev_trader_cnt);
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_a_tradable_price_received_then_trade_board_updated(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            4.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // get previous state of trade_board
        let prev_trade: Trade;
        let cur_trade: Trade;
        let tm = Utc::now();
        if let Some(res) = scheduler.trade_board.get_back_trade(&Card::Bulbasaur) {
            prev_trade = res.clone();
        } else {
            prev_trade = Trade::new(tm.clone(), 1, 1, 1.00, 1);
        }

        // process the request
        scheduler.process(&req);

        // get current state of trade_board

        if let Some(res) = scheduler.trade_board.get_back_trade(&Card::Bulbasaur) {
            cur_trade = res.clone();
        } else {
            cur_trade = Trade::new(tm.clone(), 1, 1, 1.00, 1);
        }

        assert_ne!(cur_trade, prev_trade);
        assert_eq!(&5, cur_trade.get_buy_side_id());
        assert_eq!(&6, cur_trade.get_sell_side_id());
        assert_eq!(&5.00, cur_trade.get_tx_price());
        assert_eq!(&1, cur_trade.get_tx_vol());
    }

    #[test]
    fn given_there_are_buy_orders_when_a_sell_order_with_a_tradable_price_received_then_status_board_updated(
    ) {
        let mut scheduler = Scheduler::new();
        for i in 1..6 {
            let (uuid, tm, side, order_px, vol, card, trade_id) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                (0.00 + i as f64),
                1,
                Card::Bulbasaur,
                i,
            );
            let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);
            scheduler.tx_board.add_tx_req(&req);
            // update status board
            let stats = Stats::new(
                req.get_uuid(),
                req.get_tm(),
                req.get_side(),
                req.get_order_px(),
                req.get_vol(),
                req.get_card(),
                OrderStatus::Confirmed,
            );
            scheduler
                .status_board
                .add_status(req.get_trade_id(), req.get_uuid(), stats);
        }

        // generate sell order
        let (uuid, tm, side, order_px, vol, card, trade_id) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Sell,
            4.00,
            1,
            Card::Bulbasaur,
            6,
        );
        let req = RequestOrder::new(uuid, tm, side, order_px, vol, card, trade_id);

        // get previous state of status board
        let prev_buy_side_uuid: Uuid;
        let prev_buy_side_stat: Stats;
        if let Some(res) = scheduler.status_board.get_back_uuid(&5) {
            prev_buy_side_uuid = res.clone();
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler.status_board.get_stat(&5, &prev_buy_side_uuid) {
            prev_buy_side_stat = res.clone();
            assert_eq!(&OrderStatus::Confirmed, prev_buy_side_stat.get_status());
        } else {
            panic!("[ERROR] Test Failed: stat does not exist in status_board");
        }

        // process order request
        scheduler.process(&req);

        // get current state of status board
        let cur_buy_side_stat: Stats;
        let cur_sell_side_stat: Stats;
        let cur_sell_side_uuid: Uuid;
        if let Some(res) = scheduler.status_board.get_stat(&5, &prev_buy_side_uuid) {
            cur_buy_side_stat = res.clone();
            assert_eq!(&OrderStatus::Filled, cur_buy_side_stat.get_status());
        } else {
            panic!("[ERROR] Test Failed: stat does not exist in status_board");
        }

        if let Some(res) = scheduler.status_board.get_back_uuid(&6) {
            cur_sell_side_uuid = res.clone();
            assert_eq!(
                Some(cur_sell_side_uuid),
                scheduler.status_board.get_back_uuid(&6)
            );
        } else {
            panic!("[ERROR] Test Failed: uuid does not exist in status_list");
        }

        if let Some(res) = scheduler.status_board.get_stat(&6, &cur_sell_side_uuid) {
            cur_sell_side_stat = res.clone();
            assert_eq!(&OrderStatus::Filled, cur_sell_side_stat.get_status());
        } else {
            panic!("[ERROR] Test Failed: stat does not exist in status_board");
        }
    }
}
