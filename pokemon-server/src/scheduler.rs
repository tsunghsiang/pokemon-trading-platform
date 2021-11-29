use crate::data_type::RequestOrder;
use crate::trade_board::TradeBoard;
use crate::tx_board::TxBoard;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use tide::Request;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub order_queue: VecDeque<RequestOrder>,
    tx_baord: TxBoard,
    trade_board: TradeBoard,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            order_queue: VecDeque::<RequestOrder>::new(),
            tx_baord: TxBoard::new(),
            trade_board: TradeBoard::new(),
        }
    }
}
