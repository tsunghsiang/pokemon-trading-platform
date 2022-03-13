#[macro_use]
extern crate ini;

use std::sync::atomic::{AtomicBool, Ordering};
use data_type::{Card, RequestOrder, ProcessStatus, Rsp, HistoryParam};
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use tide::{Body, Request, Response, StatusCode};
use std::env;
use settings::Settings;
use std::thread;
use std::time::Duration;
use ctrlc;
use chrono::Local;

mod settings;
mod data_type;
mod scheduler;
mod status_board;
mod trade_board;
mod tx_board;
mod database;

static STOP: AtomicBool = AtomicBool::new(false);

pub fn get_server_config(mut args: env::Args) -> String {
    match args.nth(1) {
        Some(config) => {
            let cfg = Settings::new(config);
            cfg.get_server_url()
        },
        None => {
            String::from("localhost:8080")
        }
    }
}

pub fn order_queue_proc(schler: &std::sync::Arc<std::sync::Mutex<scheduler::Scheduler>>) {
    let handler = Arc::clone(&schler);
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
}

pub fn shudown_checker(handler: &std::sync::Arc<std::sync::Mutex<scheduler::Scheduler>>) {
    let mut check_times: i32 = 0;
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
                thread::sleep(Duration::from_millis(100));
            },
            Err(err) => {
                eprintln!("[ERROR] {}", err);
            }
        }
    }
}

pub fn signal_handler(terminator: &std::sync::Arc<std::sync::Mutex<scheduler::Scheduler>>) {
    STOP.store(true, Ordering::Release);
    let handler = Arc::clone(&terminator);
    println!("\n[SHUTDOWN] Server shutting down. Consuming rest requests");
    std::thread::spawn(move || shudown_checker(&handler));    
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Obtain config file path
    let args = env::args();
    if args.len() != 2 {
        panic!("Usage: ./[executable] [config_file_path]");
    }

    // Set server configurations
    let srv = get_server_config(args);

    // Recover transaction board if there are interruptions during a day
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    scheduler.lock().unwrap().recover();
    
    let (req_checker, trade_checker, order_checker, activator, terminator, trade_history, order_history, order_status) = (
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
        scheduler.clone(),
    );

    let mut server = tide::new();

    // Spawn process of an order queue
    std::thread::spawn(move || order_queue_proc(&activator));

    // Graceful shutdown handler
    ctrlc::set_handler(move || signal_handler(&terminator)).expect("Error setting Ctrl-C handler");

    server
        .at("/api/pokemon/card")
        .post(move |mut req: Request<()>| {
            let handler = Arc::clone(&req_checker);
            async move {
                if !STOP.load(Ordering::Acquire) {
                    let order: RequestOrder = req.body_json().await?;
                    let mut res = Response::new(StatusCode::Ok);
                    let rsp = Rsp::<RequestOrder>::new(ProcessStatus::Success, String::from("Processed"), order);
                    handler.lock().unwrap().order_queue.push_back(order);
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                } else {
                    let mut res = Response::new(StatusCode::BadGateway);
                    let rsp = Rsp::<String>::new(ProcessStatus::Failed, String::from("Server shutting down. Stop serving requests"), String::from("{}"));
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                }
            }
        });

    server
        .at("/api/pokemon/trade/:card")
        .get(move |req: Request<()>| {
            let handler = Arc::clone(&trade_checker);
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

                    let mut data = String::from("");
                    if let Some(list) = handler.lock().unwrap().get_latest_trades(&param) {
                        if list.len() > 0 {
                            for elem in list {
                                data += &elem.to_str();
                                data.push(',');
                            }
                        }
                    }

                    let msg = format!("view the latest 50 trades on card - {:?}", param);
                    let mut res = Response::new(StatusCode::Ok);
                    let rsp = Rsp::<String>::new(ProcessStatus::Success, msg, format!("[{}]", data));
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                } else {
                    let mut res = Response::new(StatusCode::BadGateway);
                    let rsp = Rsp::<String>::new(ProcessStatus::Failed, String::from("Server shutting down. Stop serving requests"), String::from("[]"));
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                }
            }
        });

    server
        .at("/api/pokemon/order/:id")
        .get(move |req: Request<()>| {
            let handler = Arc::clone(&order_checker);
            async move {
                if !STOP.load(Ordering::Acquire) {
                    let id: i32 = 0;
                    if let Ok(s) = req.param("id") {
                        if let Ok(id) = s.parse::<i32>() {

                        } else {
                            let mut res = Response::new(StatusCode::BadRequest);
                            let rsp = Rsp::<String>::new(ProcessStatus::Failed, String::from("Digit Parsed Error"), String::from("[]"));
                            res.set_body(Body::from_json(&rsp)?);
                            return Ok(res)
                        }
                    } else {
                        let mut res = Response::new(StatusCode::BadRequest);
                        let rsp = Rsp::<String>::new(ProcessStatus::Failed, String::from("InvalidDigit"), String::from("[]"));
                        res.set_body(Body::from_json(&rsp)?);
                        return Ok(res)
                    }

                    let mut data = String::from("");
                    if let Some(stats) = handler.lock().unwrap().get_latest_orders(&id) {             
                        if stats.len() > 0 {
                            for elem in stats {
                                data.push_str(&elem.to_str());
                                data.push(',');
                            }               
                        }
                    }

                    let msg = format!("view the status of latest 50 orders of trader {}", id);
                    let mut res = Response::new(StatusCode::Ok);
                    let rsp = Rsp::<String>::new(ProcessStatus::Success, msg, format!("[{}]", data));
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                } else {
                    let mut res = Response::new(StatusCode::BadGateway);
                    let rsp = Rsp::<String>::new(ProcessStatus::Failed, String::from("Server shutting down. Stop serving requests"), String::from("[]"));
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                }
            }
        });

    server
        .at("/api/pokemon/trade/history")
        .get(move |req: Request<()>|{
            let handler = Arc::clone(&trade_history);
            async move {
                if !STOP.load(Ordering::Acquire) {

                    match req.query::<HistoryParam>() {
                        Ok(param) => {
                            let mut res = Response::new(StatusCode::Ok);
                            let msg = format!("view the trade history of trader {} on {}", param.get_id(), param.get_date());

                            if let Some(history) = handler.lock().unwrap().get_trade_history(param.get_id(), param.get_date()) {
                                let mut data = String::from("");
                                for elem in history {
                                    data.push_str(&elem.to_str());
                                    data.push(',');
                                }
                                let rsp = Rsp::<String>::new(ProcessStatus::Success, msg, format!("[{}]", data));
                                res.set_body(Body::from_json(&rsp)?);
                            } else {
                                let rsp = Rsp::<String>::new(ProcessStatus::Success, msg, String::from("[]"));
                                res.set_body(Body::from_json(&rsp)?);
                            }
                            
                            Ok(res)
                        },
                        Err(e) => {
                            let mut res = Response::new(StatusCode::BadRequest);
                            let rsp = Rsp::<String>::new(ProcessStatus::Failed, e.to_string(), String::from("[]"));
                            res.set_body(Body::from_json(&rsp)?);
                            Ok(res)
                        }
                    }

                } else {
                    let mut res = Response::new(StatusCode::BadGateway);
                    let rsp = Rsp::<String>::new(ProcessStatus::Failed, String::from("Server shutting down. Stop serving requests"), String::from("[]"));
                    res.set_body(Body::from_json(&rsp)?);
                    Ok(res)
                }
            }
        });
    
    server.listen(srv).await?;
    Ok(())
}