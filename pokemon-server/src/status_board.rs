use crate::data_type::{Card, OrderStatus, Side};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, LinkedList};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Stats {
    uuid: Uuid,
    tm: DateTime<Utc>,
    side: Side,
    order_px: f64,
    vol: i32,
    card: Card,
    status: OrderStatus,
}

impl Stats {
    pub fn new(
        uuid: Uuid,
        tm: DateTime<Utc>,
        side: Side,
        order_px: f64,
        vol: i32,
        card: Card,
        status: OrderStatus,
    ) -> Self {
        Self {
            uuid,
            tm,
            side,
            order_px,
            vol,
            card,
            status,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBoard {
    // Hash: id -> latest 50 orders
    status_board: HashMap<i32, HashMap<Uuid, Stats>>,
    status_list: HashMap<i32, LinkedList<Uuid>>,
    limit: i32,
}

impl StatusBoard {
    pub fn new() -> Self {
        let board = HashMap::<i32, HashMap<Uuid, Stats>>::new();
        let list = HashMap::<i32, LinkedList<Uuid>>::new();
        Self {
            status_board: board,
            status_list: list,
            limit: 50,
        }
    }

    pub fn update_status(&mut self, id: i32, uuid: Uuid, status: OrderStatus) {
        if let Some(res) = self.status_board.get_mut(&id) {
            if let Some(stats) = res.get_mut(&uuid) {
                stats.status = status;
            }
        }
    }

    pub fn add_status(&mut self, id: i32, uuid: Uuid, stats: Stats) {
        // add new one into status_board
        if let Some(res) = self.status_board.get_mut(&id) {
            res.insert(uuid, stats);
        } else {
            self.status_board.insert(id, HashMap::<Uuid, Stats>::new());
            if let Some(res) = self.status_board.get_mut(&id) {
                res.insert(uuid, stats);
            }
        }

        // add new one into status_list
        if let Some(res) = self.status_list.get_mut(&id) {
            res.push_back(uuid);
        } else {
            self.status_list.insert(id, LinkedList::<Uuid>::new());
            if let Some(res) = self.status_list.get_mut(&id) {
                res.push_back(uuid);
            }
        }
    }
}
