use crate::data_type::{ProcessResult, RequestOrder};
use crate::status_board::StatusBoard;
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
    status_board: StatusBoard,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            order_queue: VecDeque::<RequestOrder>::new(),
            tx_baord: TxBoard::new(),
            trade_board: TradeBoard::new(),
            status_board: StatusBoard::new(),
        }
    }

    //pub fn process(req: &RequestOrder) -> ProcessResult {

    //    ProcessResult::TxConfirmed
    //}
}
