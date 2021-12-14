use crate::data_type::{Card, OrderStatus, Side};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, LinkedList};
use std::option::Option;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn get_tm(&self) -> &DateTime<Utc> {
        &self.tm
    }

    pub fn get_side(&self) -> &Side {
        &self.side
    }

    pub fn get_order_px(&self) -> &f64 {
        &self.order_px
    }

    pub fn get_vol(&self) -> &i32 {
        &self.vol
    }

    pub fn get_card(&self) -> &Card {
        &self.card
    }

    pub fn get_status(&self) -> &OrderStatus {
        &self.status
    }
}

#[derive(Debug, Clone)]
pub struct StatusBoard {
    // Hash: id -> latest 50 orders
    status_board: HashMap<i32, HashMap<Uuid, Stats>>,
    status_list: HashMap<i32, LinkedList<Uuid>>,
    limit: usize,
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
            if res.len() < self.limit {
                res.push_back(uuid);
            } else {
                res.pop_front();
                res.push_back(uuid);
            }
        } else {
            self.status_list.insert(id, LinkedList::<Uuid>::new());
            if let Some(res) = self.status_list.get_mut(&id) {
                res.push_back(uuid);
            }
        }
    }

    pub fn update_status(&mut self, id: i32, uuid: Uuid, status: OrderStatus) {
        if let Some(res) = self.status_board.get_mut(&id) {
            if let Some(stats) = res.get_mut(&uuid) {
                stats.status = status;
            }
        }
    }

    pub fn get_limit(&self) -> &usize {
        &self.limit
    }

    pub fn get_back_uuid(&self, id: &i32) -> Option<Uuid> {
        let res = match self.status_list.get(id) {
            Some(list) => {
                if let Some(e) = list.back() {
                    Some(e.clone())
                } else {
                    None
                }
            }
            None => None,
        };

        res
    }

    pub fn get_front_uuid(&self, id: &i32) -> Option<Uuid> {
        let res = match self.status_list.get(id) {
            Some(list) => {
                if let Some(e) = list.front() {
                    Some(e.clone())
                } else {
                    None
                }
            }
            None => None,
        };

        res
    }

    pub fn get_stat(&self, id: &i32, uuid: &Uuid) -> Option<Stats> {
        let res = match self.status_board.get(id) {
            Some(content) => {
                if let Some(e) = content.get(uuid) {
                    Some(e.clone())
                } else {
                    None
                }
            }
            None => None,
        };

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{Card, OrderStatus, Side};
    use crate::status_board::{Stats, StatusBoard};
    use chrono::Utc;
    use rand::Rng;
    use uuid::Uuid;

    #[test]
    fn given_fields_provided_when_stats_instatiated_then_all_fields_accessible() {
        let (uuid, tm, side, order_px, vol, card, status) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            1.00,
            1,
            Card::Bulbasaur,
            OrderStatus::Confirmed,
        );

        let (
            expected_uuid,
            expected_tm,
            expected_side,
            expected_order_px,
            expected_vol,
            expected_card,
            expected_status,
        ) = (
            uuid.clone(),
            tm.clone(),
            side.clone(),
            order_px.clone(),
            vol.clone(),
            card.clone(),
            status.clone(),
        );

        let stat = Stats::new(uuid, tm, side, order_px, vol, card, status);
        assert_eq!(&expected_uuid, stat.get_uuid());
        assert_eq!(&expected_tm, stat.get_tm());
        assert_eq!(&expected_side, stat.get_side());
        assert_eq!(&expected_order_px, stat.get_order_px());
        assert_eq!(&expected_vol, stat.get_vol());
        assert_eq!(&expected_card, stat.get_card());
        assert_eq!(&expected_status, stat.get_status());
    }

    #[test]
    fn given_limit_configured_when_status_board_instatiated_then_limit_accessible() {
        let board = StatusBoard::new();
        assert_eq!(&50, board.get_limit());
    }

    #[test]
    fn given_limit_unreached_when_a_stat_added_then_it_could_be_obtained_in_board_and_list() {
        let (uuid, tm, side, order_px, vol, card, status) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            1.00,
            1,
            Card::Bulbasaur,
            OrderStatus::Confirmed,
        );
        let stat = Stats::new(uuid, tm, side, order_px, vol, card, status);
        let expected_stat = stat.clone();
        let mut board = StatusBoard::new();
        let (id, uuid) = (1, uuid.clone());

        // add a stat into status board
        board.add_status(id, uuid, stat);

        // check if uuid exists in status_list
        if let Some(res) = board.get_back_uuid(&id) {
            assert_eq!(uuid, res);
        } else {
            panic!("[ERROR] Test Failed: stat is not successfully inserted");
        }

        // check if stat exists in status_board
        if let Some(res) = board.get_stat(&id, &uuid) {
            assert_eq!(expected_stat, res);
        } else {
            panic!("[ERROR] Test Failed: stat is not successfully inserted");
        }
    }

    #[test]
    fn given_limit_reached_when_a_stat_added_then_front_elem_of_status_list_is_popped_and_it_is_accessible_in_status_board(
    ) {
        let mut board = StatusBoard::new();
        let mut rng = rand::thread_rng();
        for _ in 1..51 {
            let (uuid, tm, side, order_px, vol, card, status) = (
                Uuid::new_v4(),
                Utc::now(),
                Side::Buy,
                rng.gen_range(1.00..11.00),
                rng.gen_range(1..11),
                Card::Bulbasaur,
                OrderStatus::Confirmed,
            );
            let stat = Stats::new(uuid, tm, side, order_px, vol, card, status);
            // here we specify trade_id = 1
            board.add_status(1, uuid, stat);
        }

        // obtained front uuid in status_list for later comparison
        let front = board.get_front_uuid(&1);
        // add a stat into status board
        let (uuid, tm, side, order_px, vol, card, status) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            rng.gen_range(1.00..11.00),
            rng.gen_range(1..11),
            Card::Bulbasaur,
            OrderStatus::Confirmed,
        );
        let stat = Stats::new(uuid, tm, side, order_px, vol, card, status);
        let expected_stat = Some(stat.clone());
        board.add_status(1, uuid, stat);
        // check if front uuid in status_list has been popped
        let new_front = board.get_front_uuid(&1);
        assert_ne!(front.unwrap(), new_front.unwrap());
        // check if a newly-added stat has been inserted into a status board
        let real_stat = board.get_stat(&1, &uuid);
        assert_eq!(expected_stat, real_stat);
    }

    #[test]
    fn given_status_is_updated_when_read_then_could_be_verified() {
        let mut board = StatusBoard::new();
        let (uuid, tm, side, order_px, vol, card, status) = (
            Uuid::new_v4(),
            Utc::now(),
            Side::Buy,
            2.00,
            5,
            Card::Bulbasaur,
            OrderStatus::Confirmed,
        );
        let stat = Stats::new(uuid, tm, side, order_px, vol, card, status);
        board.add_status(1, uuid, stat);

        board.update_status(1, uuid, OrderStatus::Filled);

        if let Some(res) = board.get_stat(&1, &uuid) {
            assert_eq!(&OrderStatus::Filled, res.get_status());
        } else {
            panic!("[ERROR] Test Failed: Stat should be found but none.");
        }
    }
}
