use std::env;
use std::process::exit;
use std::thread;
use trader::Trader;

mod data_type;
mod trader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[ERR] ./[executable] [num_of_traders]");
        exit(-1);
    }

    let num: i32 = args[1].parse::<i32>().unwrap();
    for i in 0..num {
        thread::spawn(move || {
            let trader = Trader::new(i);
            trader.send_request();
        });
    }
    loop {}
}
