#[macro_use]
extern crate ini;

use std::env;
use std::thread;
use trader::Trader;
use settings::Settings;

mod settings;
mod data_type;
mod trader;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("Usage: ./[executable] [config_file_path]");
    }

    let mut cfg = Settings::init();
    if let Some(config) = args.nth(1) {
        println!("{}", config);
        cfg = Settings::set(config);
    } else {
        panic!("[ERROR] unknown configuration path");
    }

    let num: i32 = cfg.get_trader_nums();
    for i in 0..num {
        let property = cfg.clone();
        thread::spawn(move || {
            let trader = Trader::new(i, property.clone());
            trader.send_request();
        });
    }
    loop {}
}
