use crate::data_type::RequestOrder;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use tide::Request;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub order_queue: VecDeque<RequestOrder>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            order_queue: VecDeque::<RequestOrder>::new(),
        }
    }
}
