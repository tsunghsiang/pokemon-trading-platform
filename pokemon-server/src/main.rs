use data_type::{Card, RequestOrder};
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use tide::Request;

mod data_type;
mod scheduler;
mod status_board;
mod trade_board;
mod tx_board;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    let (req_checker, trade_checker, order_checker, activator) = (
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

    server
        .at("/api/pokemon/card")
        .post(move |mut req: Request<()>| {
            let handler = Arc::clone(&req_checker);
            async move {
                let order: RequestOrder = req.body_json().await?;
                let res = format!("Confirmed: {:?}", &order);
                handler.lock().unwrap().order_queue.push_back(order);
                Ok(res)
            }
        });

    server
        .at("/api/pokemon/trade/:card")
        .get(move |req: Request<()>| {
            let handler = Arc::clone(&trade_checker.clone());
            async move {
                let card = req.param("card").unwrap_or("None");
                let param = match card {
                    "Pikachu" => Card::Pikachu,
                    "Bulbasaur" => Card::Bulbasaur,
                    "Charmander" => Card::Charmander,
                    "Squirtle" => Card::Squirtle,
                    _ => Card::Pikachu,
                };

                let mut res = String::from("");
                if let Some(list) = handler.lock().unwrap().get_latest_trades_on_cards(&param) {
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
            }
        });

    server
        .at("/api/pokemon/order/:id")
        .get(move |req: Request<()>| {
            let handler = Arc::clone(&order_checker.clone());
            async move {
                let id = req.param("id").unwrap();
                let res = format!("View the latest 50 orders of trader[{}]", &id);
                Ok(res)
            }
        });

    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
