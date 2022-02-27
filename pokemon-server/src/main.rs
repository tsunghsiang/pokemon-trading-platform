#[macro_use]
extern crate ini;

use std::sync::atomic::{AtomicBool, Ordering};
use data_type::{Card, RequestOrder};
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use tide::Request;
use std::env;
use settings::Settings;
use std::{thread, time};
use ctrlc;

mod settings;
mod data_type;
mod scheduler;
mod status_board;
mod trade_board;
mod tx_board;
mod database;

static STOP: AtomicBool = AtomicBool::new(false);

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Obtain config file path
    let mut args = env::args();
    if args.len() != 2 {
        panic!("Usage: ./[executable] [config_file_path]");
    }

    // Set server configurations
    let mut srv = match args.nth(1) {
        Some(config) => {
            let cfg = Settings::new(config);
            cfg.get_server_url()
        },
        None => {
            String::from("localhost:8080")
        }
    };

    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    scheduler.lock().unwrap().recover();
    
    let (req_checker, trade_checker, order_checker, activator, terminator) = (
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
    );

    let mut server = tide::new();

    std::thread::spawn(move || {
        let handler = Arc::clone(&activator);
        loop {
            match handler.lock() {
                Ok(mut res) => {
                    if let Some(req) = res.order_queue.pop_front() {
                        res.process(&req);
                        // println!("{:?}", &req);
                    }
                }
                Err(err) => {
                    eprintln!("[ERROR] {}", err);
                }
            }
        }
    });

    // Graceful shutdown handler
    ctrlc::set_handler(move || {
        STOP.store(true, Ordering::Release);
        let handler = Arc::clone(&terminator);
        let mut check_times: i32 = 0;
        println!("[SHUTDOWN] Server shutting down. Consuming rest requests");
        
        std::thread::spawn(move || {
            loop {
                match handler.lock() {
                    Ok(res) => {
                        let livings: usize = res.order_queue.len();
                        if livings == 0 {
                            check_times = check_times + 1;
                            if check_times >= 10 {
                                println!("[SHUTDOWN] Server shutdown.");
                                std::process::exit(-1);
                            }
                        } else {
                            println!("[SHUTDOWN] Server shutting down. Consuming rest requests");
                        }
                        thread::sleep_ms(100);
                    },
                    Err(err) => {
                        eprintln!("[ERROR] {}", err);
                    }
                }
            }            
        });
    }).expect("Error setting Ctrl-C handler");

    server
        .at("/api/pokemon/card")
        .post(move |mut req: Request<()>| {
            let handler = Arc::clone(&req_checker);
            async move {
                if !STOP.load(Ordering::Acquire) {
                    let order: RequestOrder = req.body_json().await?;
                    let res = format!("Confirmed: {:?}", &order);
                    handler.lock().unwrap().order_queue.push_back(order);
                    Ok(res)
                } else {
                    let res = format!("[SHUTDOWN] Server shutting down. Stop serving requests");
                    Ok(res)
                }
            }
        });

    server
        .at("/api/pokemon/trade/:card")
        .get(move |req: Request<()>| {
            let handler = Arc::clone(&trade_checker.clone());
            async move {
                if !STOP.load(Ordering::Acquire) {
                    let card = req.param("card").unwrap_or("None");
                    let param = match card {
                        "Pikachu" => Card::Pikachu,
                        "Bulbasaur" => Card::Bulbasaur,
                        "Charmander" => Card::Charmander,
                        "Squirtle" => Card::Squirtle,
                        _ => Card::Pikachu,
                    };

                    let mut res = String::from("");
                    if let Some(list) = handler.lock().unwrap().get_latest_trades(&param) {
                        let header_begin = format!(
                            "============================================= {:?} =============================================",
                            param
                        );
                        let header_end = format!(
                            "\n============================================= {:?} =============================================",
                            param
                        );

                        res += &header_begin;

                        if list.len() > 0 {
                            for elem in list {
                                let row = format!(
                                    "\n|{}|buy_side_id: {}|sell_side_id: {}|px: {}|vol: {}|",
                                    elem.get_tx_time(),
                                    elem.get_buy_side_id(),
                                    elem.get_sell_side_id(),
                                    elem.get_tx_price(),
                                    elem.get_tx_vol(),
                                );
                                res += &row;
                            }
                        } else {
                            res += "\nNone";
                        }

                        res += &header_end;
                    }

                    Ok(res.to_string())
                } else {
                    let res = format!("[SHUTDOWN] Server shutting down. Stop serving requests");
                    Ok(res)
                }
            }
        });

    server
        .at("/api/pokemon/order/:id")
        .get(move |req: Request<()>| {
            let handler = Arc::clone(&order_checker.clone());
            async move {
                if !STOP.load(Ordering::Acquire) {
                    let id = req.param("id").unwrap().parse::<i32>().unwrap();
                    let mut res = String::from("");

                    if let Some(stats) = handler.lock().unwrap().get_latest_orders(&id) {
                    
                        let header_begin = format!(
                            "============================================= Trader[{}] =============================================",
                            &id
                        );
                        let header_end = format!(
                            "\n============================================= Trader[{}] =============================================",
                            &id
                        );

                        res += &header_begin;               

                        if stats.len() > 0 {
                            for elem in stats {
                                let row = format!(
                                    "\n|{}|side: {:?}|order_px: {}|vol: {}|card: {:?}|status: {:?}|",
                                    elem.get_tm(),
                                    elem.get_side(),
                                    elem.get_order_px(),
                                    elem.get_vol(),
                                    elem.get_card(),
                                    elem.get_status()
                                );
                                res += &row;
                            }               
                        } else {
                            res += "\nNone";
                        }
                        
                        res += &header_end;
                    }

                    Ok(res.to_string())
                } else {
                    let res = format!("[SHUTDOWN] Server shutting down. Stop serving requests");
                    Ok(res)
                }
            }
        });

    server.listen(srv).await?;
    Ok(())
}