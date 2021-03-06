use crate::data_type::{Card, Side};
use crate::settings::Settings;
use chrono::prelude::*;
use chrono::Local;
use rand::Rng;
use std::{thread, time};
use uuid::Uuid;

pub struct Trader {
    id: i32,
    config: Settings,
}

impl Trader {
    pub fn new(id: i32, config: Settings) -> Self {
        Trader { id, config }
    }

    pub fn send_request(&self) {
        let wait_tm = time::Duration::from_millis(1000);
        loop {
            let op: i32 = rand::thread_rng().gen_range(0..5);
            match op {
                0 => {
                    match self.post_order() {
                        Ok(_) => {},
                        Err(e) => { println!("Kind: {}", e.kind()); }
                    }
                },
                1 => {
                    match self.get_trade() {
                        Ok(_) => {},
                        Err(e) => { println!("Kind: {}", e.kind()); }
                    }
                },
                2 => {
                    match self.get_order() {
                        Ok(_) => {},
                        Err(e) => { println!("Kind: {}", e.kind()); }
                    }
                },
                3 => {
                    match self.get_trade_record() {
                        Ok(_) => {},
                        Err(e) => { println!("Kind: {}", e.kind()); }
                    }
                },
                4 => {
                    match self.get_request_record() {
                        Ok(_) => {},
                        Err(e) => { println!("Kind: {}", e.kind()); }
                    }
                },
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
        let url = format!("http://{}/api/pokemon/card", &self.config.get_connected_url());
        let rsp: String = ureq::post(url.as_str())
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
        let url: String = format!("http://{}/api/pokemon/trade/{}", &self.config.get_connected_url(), card);
        let rsp: String = ureq::get(&url).call()?.into_string()?;
        println!("{}", &rsp);
        Ok(())
    }

    fn get_order(&self) -> Result<(), ureq::Error> {
        let url: String = format!("http://{}/api/pokemon/order/{}", &self.config.get_connected_url(), &self.id);
        let rsp: String = ureq::get(&url).call()?.into_string()?;
        println!("{}", &rsp);
        Ok(())
    }

    fn get_trade_record(&self) -> Result<(), ureq::Error> {
        let tm = Local::now();
        let date = tm.format("%Y-%m-%d");
        let url: String = format!("http://{}/api/pokemon/trade/history?id={}&date={}", &self.config.get_connected_url(), self.id, date);
        let rsp: String = ureq::get(&url).call()?.into_string()?;
        println!("{}", &rsp);
        Ok(())        
    }

    fn get_request_record(&self) -> Result<(), ureq::Error> {
        let tm = Local::now();
        let date = tm.format("%Y-%m-%d");
        let url: String = format!("http://{}/api/pokemon/request/history?id={}&date={}", &self.config.get_connected_url(), self.id, date);
        let rsp: String = ureq::get(&url).call()?.into_string()?;
        println!("{}", &rsp);
        Ok(())        
    }
}
