use crate::data_type::{OrderStatus, ProcessResult, RequestOrder, Side};
use crate::status_board::{Stats, StatusBoard};
use crate::trade_board::{Trade, TradeBoard};
use crate::tx_board::{Tag, TxBoard};
use chrono::Utc;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use tide::Request;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub order_queue: VecDeque<RequestOrder>,
    tx_board: TxBoard,
    trade_board: TradeBoard,
    status_board: StatusBoard,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            order_queue: VecDeque::<RequestOrder>::new(),
            tx_board: TxBoard::new(),
            trade_board: TradeBoard::new(),
            status_board: StatusBoard::new(),
        }
    }

    pub fn process(&mut self, req: &RequestOrder) -> ProcessResult {
        let mut proc_res: ProcessResult = ProcessResult::TxConfirmed;
        let card = req.get_card();

        if let Some(res) = self.tx_board.get_board_content().get_mut(&card) {
            proc_res = match req.get_side() {
                Side::Buy => {
                    let mut px = 1;
                    let sell_card_board = res.get_bs_board(Side::Sell);
                    let mut px = 1;
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
                                    return ProcessResult::TxBoardUpdateFail;
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
                                return ProcessResult::TxFilled;
                            } else {
                                px += 1;
                            }
                        }

                        if px > 10 {
                            // update tx_board
                            let buy_card_board = res.get_bs_board(Side::Buy);
                            let tag = Tag::new(req.get_trade_id(), req.get_uuid());
                            if let Some(cur_vol) = buy_card_board.get_mut(&req.get_trade_id()) {
                                cur_vol.set_vol(cur_vol.get_vol() + req.get_vol());
                                cur_vol.push_trader(tag);
                            }
                            println!("update_tx_baord");
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
                            println!("update status board");
                            return ProcessResult::TxConfirmed;
                        }
                    }
                }
                Side::Sell => ProcessResult::TxConfirmed,
            };
        } else {
            return ProcessResult::UnknownCard;
        }
        proc_res
    }
}
