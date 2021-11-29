use data_type::RequestOrder;
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use tide::Request;

mod data_type;
mod scheduler;
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
                        println!("{:?}", &req);
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
                let res = format!("View the latest 50 trades on card: {}", &card);
                Ok(res)
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
