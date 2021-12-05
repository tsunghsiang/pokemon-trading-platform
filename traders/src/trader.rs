use crate::data_type::{Card, Side};
use chrono::prelude::*;
use rand::Rng;
use std::{thread, time};
use uuid::Uuid;

pub struct Trader {
    id: i32,
}

impl Trader {
    pub fn new(id: i32) -> Self {
        Trader { id }
    }

    pub fn send_request(&self) {
        let wait_tm = time::Duration::from_millis(1000);
        loop {
            let op: i32 = rand::thread_rng().gen_range(0..3);
            match op {
                0 => self.post_order().unwrap(),
                1 => self.get_trade().unwrap(),
                2 => self.get_order().unwrap(),
                _ => {}
            }
            thread::sleep(wait_tm);
        }
    }

    fn post_order(&self) -> Result<(), ureq::Error> {
        let uuid = Uuid::new_v4();
        let tm: DateTime<Utc> = Utc::now();
        let side = match rand::thread_rng().gen_range(0..2) {
            0 => Side::Buy,
            1 => Side::Sell,
            _ => Side::Buy,
        };
        let order_px = rand::thread_rng().gen_range(1..11) as f64;
        let vol: i32 = 1;
        let card = match rand::thread_rng().gen_range(0..4) {
            0 => Card::Pikachu,
            1 => Card::Bulbasaur,
            2 => Card::Charmander,
            3 => Card::Squirtle,
            _ => Card::Pikachu,
        };
        let rsp: String = ureq::post("http://127.0.0.1:8080/api/pokemon/card")
            .set("Content-type", "application/json")
            .send_json(ureq::json!({
                "uuid": uuid,
                "tm": tm,
                "side": side,
                "order_px": order_px,
                "vol": vol,
                "card": card,
                "trader_id": &self.id
            }))?
            .into_string()?;
        println!("{}", &rsp);
        Ok(())
    }

    fn get_trade(&self) -> Result<(), ureq::Error> {
        let card: &str = match rand::thread_rng().gen_range(0..4) {
            0 => "Pikachu",
            1 => "Bulbasaur",
            2 => "Charmander",
            3 => "Squirtle",
            _ => "",
        };
        let url: String = format!("http://127.0.0.1:8080/api/pokemon/trade/{}", card);
        let rsp: String = ureq::get(&url).call()?.into_string()?;
        println!("{}", &rsp);
        Ok(())
    }

    fn get_order(&self) -> Result<(), ureq::Error> {
        let url: String = format!("http://127.0.0.1:8080/api/pokemon/order/{}", &self.id);
        let rsp: String = ureq::get(&url).call()?.into_string()?;
        println!("{}", &rsp);
        Ok(())
    }
}
