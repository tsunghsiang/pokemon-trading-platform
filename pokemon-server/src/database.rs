use crate::data_type::{Card, OrderStatus, RequestOrder, Side};
use crate::settings::Settings;
use contracts::*;
use postgres::{Client, NoTls, Row};
use uuid::Uuid;
use std::env;

pub struct Database {
    client: Client,
}

impl Database {
    #[ensures(ret.is_connected() == true, "database is connected")]
    pub fn new() -> Self {
        // Obtain config file path
        let mut args = env::args();
        let database = match args.nth(1) {
            Some(config) => {
                let cfg = Settings::new(config);
                cfg.get_database_url()
            },
            None => {
                String::from("postgresql://postgres:test@localhost:5432/pokemon")
            }
        };

        let mut db = Database {
            client: Client::connect(
                database.as_str(),
                NoTls,
            )
            .unwrap(),
        };

        db.init_tables();
        db
    }
}

impl Database {
    #[requires(self.is_connected(), "database should be connected")]
    #[ensures(self.enum_type_exist("side"), "enum Side should be created after the database initialization")]
    #[ensures(self.enum_type_exist("card"), "enum Card should be created after the database initialization")]
    #[ensures(self.enum_type_exist("orderstatus"), "enum OrderStatus should be created after the database initialization")]
    #[ensures(self.table_exist("public", "request_table"), "request_table should be created after the database initialization")]
    #[ensures(self.table_exist("public", "status_table"), "status_table should be created after the database initialization")]
    #[ensures(self.table_exist("public", "trade_table"), "trade_table should be created after the database initialization")]
    #[invariant(true)]
    pub fn init_tables(&mut self) {
        // create enum 'Side'
        if !self.enum_type_exist("side") {
            self.client
                .batch_execute("CREATE TYPE side AS ENUM('Buy', 'Sell');")
                .unwrap();
        }

        // create enum 'Card'
        if !self.enum_type_exist("card") {
            self.client
                .batch_execute(
                    "CREATE TYPE card AS ENUM('Pikachu', 'Bulbasaur', 'Charmander', 'Squirtle');",
                )
                .unwrap();
        }

        // create enum 'OrderStatus'
        if !self.enum_type_exist("orderstatus") {
            self.client
                .batch_execute("CREATE TYPE orderstatus AS ENUM('Confirmed', 'Filled', 'Dropped');")
                .unwrap();
        }

        // create table 'request_table'
        self.client
            .batch_execute(
                "create table if not exists request_table(
                    uuid UUID,
                    tm timestamptz,
                    side Side,
                    order_px FLOAT8,
                    vol INTEGER,
                    card Card,
                    trader_id INTEGER);",
            )
            .unwrap();

        // create table 'status_table'
        self.client
            .batch_execute(
                "create table if not exists status_table(
                    uuid UUID,
                    status OrderStatus
                );",
            )
            .unwrap();

        // create table 'trade_table'
        self.client
            .batch_execute(
                "create table if not exists trade_table(
                    buy_uuid UUID,
                    sell_uuid UUID,
                    buy_side_id INTEGER,
                    sell_side_id INTEGER,
                    tx_price FLOAT8,
                    tx_vol INTEGER,
                    card Card
                );",
            )
            .unwrap();
    }

    pub fn is_connected(&self) -> bool {
        !self.client.is_closed()
    }

    #[requires(self.is_connected(), "database should be connected before checking whether an enum exists")]
    #[ensures(true)]
    #[invariant(true)]
    pub fn enum_type_exist(&mut self, name: &str) -> bool {
        let res = self
            .client
            .query_one(
                "select exists (select 1 from pg_type where typname = $1);",
                &[&name],
            )
            .unwrap();

        res.get("exists")
    }

    #[requires(self.is_connected(), "database should be connected before checking whether a table exists")]
    #[ensures(true)]
    #[invariant(true)]
    pub fn table_exist(&mut self, schema: &str, table: &str) -> bool {
        let res = self.client.query_one("SELECT EXISTS ( SELECT FROM information_schema.tables WHERE table_schema = $1 AND table_name = $2);", &[&schema, &table]).unwrap();
        res.get("exists")
    }

    #[requires(self.is_connected(), "database should be connected before checking whether a request exists")]
    #[requires(self.table_exist("public", "request_table"), "request_table should be created in the database")]
    #[ensures(self.request_exist(&req.get_uuid()), "the request should be inserted into request_table")]
    #[invariant(true)]
    pub fn insert_request_table(&mut self, req: &RequestOrder) {
        match self.client.execute(
            "INSERT INTO request_table(uuid, tm, side, order_px, vol, card, trader_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &req.get_uuid(),
                &req.get_tm(),
                &req.get_side(),
                &req.get_order_px(),
                &req.get_vol(),
                &req.get_card(),
                &req.get_trade_id()
            ],
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("[Database][insert_request_table] Error: {}", e);
            }
        }
    }

    #[requires(self.is_connected(), "database should be connected before checking whether a request exists")]
    #[requires(self.table_exist("public", "request_table"), "request_table should be created in the database")]
    #[ensures(true)]
    #[invariant(true)]
    pub fn request_exist(&mut self, uuid: &Uuid) -> bool {
        let res = match self.client.query_one("select uuid, tm, side, order_px, vol, card, trader_id FROM request_table where uuid = $1", &[&uuid]){
            Ok(_) => true,
            Err(_) => false,
        };
        res
    }

    #[requires(self.is_connected(), "database should be connected before inserting status of an order")]
    #[requires(self.table_exist("public", "status_table"), "status_table should be created in the database")]
    #[ensures(self.order_status_exist(uuid), "the status of the order has been inserted")]
    #[invariant(true)]
    pub fn insert_order_status(&mut self, uuid: &Uuid, status: &OrderStatus) {
        match self.client.execute(
            "insert into status_table(uuid, status) values($1, $2)",
            &[&uuid, &status],
        ) {
            Ok(n) => {}
            Err(e) => {
                panic!("[Database][insert_order_status] {}", e);
            }
        };
    }

    #[requires(self.is_connected(), "database should be connected before updating status of an order exists")]
    #[requires(self.table_exist("public", "status_table"), "status_table should be created in the database")]
    #[requires(self.order_status_exist(uuid), "status should have been existing in the status_table")]
    #[ensures(*status == self.get_order_status(uuid), "status should be the same after updated")]
    #[invariant(true)]
    pub fn update_order_status(&mut self, uuid: &Uuid, status: &OrderStatus) {
        match self.client.execute(
            "update status_table set status = $1 where uuid = $2",
            &[&status, &uuid],
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("[Database][update_order_status] {}", e);
            }
        };
    }

    #[requires(self.is_connected(), "database should be connected before checking whether status of an order exists")]
    #[requires(self.table_exist("public", "status_table"), "status_table should be created in the database")]
    #[ensures(true)]
    #[invariant(true)]
    pub fn order_status_exist(&mut self, uuid: &Uuid) -> bool {
        let res = match self.client.query_one(
            "select uuid, status FROM status_table where uuid = $1",
            &[&uuid],
        ) {
            Ok(_) => true,
            Err(_) => false,
        };
        res
    }

    #[requires(self.is_connected(), "database should be connected before getting status of an order")]
    #[requires(self.table_exist("public", "status_table"), "status_table should be created in the database")]
    #[requires(self.order_status_exist(uuid), "status should have been existing in the status_table")]
    #[ensures(true)]
    #[invariant(true)]
    pub fn get_order_status(&mut self, uuid: &Uuid) -> OrderStatus {
        let res = self
            .client
            .query_one("select status FROM status_table where uuid = $1", &[&uuid])
            .unwrap();
        res.get("status")
    }

    #[requires(self.is_connected(), "database should be connected before checking whether a request exists")]
    #[requires(self.table_exist("public", "trade_table"), "request_table should be created in the database")]
    #[ensures(self.trade_exist(&buy_side_uuid, &sell_side_uuid), "the trade should be inserted into trade_table")]
    #[invariant(true)]
    pub fn insert_trade_table(
        &mut self,
        buy_side_uuid: &Uuid,
        sell_side_uuid: &Uuid,
        buy_side_id: &i32,
        sell_side_id: &i32,
        tx_price: &f64,
        tx_vol: &i32,
        card: &Card,
    ) {
        match self.client.execute("insert into trade_table(buy_uuid, sell_uuid, buy_side_id, sell_side_id, tx_price, tx_vol, card) values($1, $2, $3, $4, $5, $6, $7)", &[&buy_side_uuid, &sell_side_uuid, &buy_side_id, &sell_side_id, &tx_price, &tx_vol, &card]){
            Ok(_) => {},
            Err(e) => { panic!("[Dtabase][insert_table_table] {}", e); }
        };
    }

    #[requires(self.is_connected(), "database should be connected before checking whether a trade exists")]
    #[requires(self.table_exist("public", "trade_table"), "trade_table should be created in the database")]
    #[ensures(true)]
    #[invariant(true)]
    pub fn trade_exist(&mut self, buy_side_uuid: &Uuid, sell_side_uuid: &Uuid) -> bool {
        let res = match self.client.query_one(
            "select * FROM trade_table where buy_uuid = $1 and sell_uuid = $2",
            &[&buy_side_uuid, &sell_side_uuid],
        ) {
            Ok(_) => true,
            Err(_) => false,
        };
        res
    }

    #[requires(self.is_connected(), "database should be connected before recovering tx_board")]
    #[requires(self.table_exist("public", "request_table"), "request_table should be created in the database")]
    #[requires(self.table_exist("public", "status_table"), "status_table should be created in the database")]
    #[ensures(true)]
    #[invariant(true)]    
    pub fn get_realtime_tx_info(&mut self, side: &Side, card: &Card) -> Vec<Row> {
        let res: Vec<Row> = self.client.query(" select rt.uuid, rt.tm, rt.side, rt.order_px, rt.vol, rt.card, rt.trader_id 
                                                from request_table rt inner join status_table st
                                                on (st.status = 'Confirmed' and 
                                                    rt.uuid = st.uuid and 
                                                    date(rt.tm) = current_date and 
                                                    rt.side = $1 and 
                                                    rt.card = $2)
                                                order by rt.tm;", &[&side, &card]).unwrap();
        res
    }

    #[requires(self.is_connected(), "database should be connected")]
    #[requires(self.table_exist("public", "request_table"), "request_table should be created in the database")]
    #[requires(self.table_exist("public", "trade_table"), "trade_table should be created in the database")]
    #[requires(id >= &0)]
    #[ensures(true)]
    #[invariant(true)] 
    pub fn get_trade_history(&mut self, id: &i32, date: &str) -> Vec<Row> {
        let res = self.client.query("select tt.buy_side_id, tt.sell_side_id, tt.tx_price, tt.tx_vol, tt.card
                                     from trade_table tt
                                     where tt.buy_uuid in ( select rt.uuid 
                                                            from request_table rt
                                                            where rt.trader_id = $1 and to_char(rt.tm, 'YYYY-MM-DD') like $2 )
                                     union
                                     select tt.buy_side_id, tt.sell_side_id, tt.tx_price, tt.tx_vol, tt.card
                                     from trade_table tt
                                     where tt.sell_uuid in ( select rt.uuid 
                                                             from request_table rt
                                                             where rt.trader_id = $1 and to_char(rt.tm, 'YYYY-MM-DD') like $2 )", &[&id, &date]).unwrap();
        res
    }

    #[requires(self.is_connected(), "database should be connected")]
    #[requires(self.table_exist("public", "request_table"), "request_table should be created in the database")]
    #[requires(id >= &0)]
    #[ensures(true)]
    #[invariant(true)] 
    pub fn get_request_history(&mut self, id: &i32, date: &str) -> Vec<Row> {
        let res = self.client.query("select * 
                                     from request_table rt
                                     where rt.trader_id = $1 and to_char(rt.tm, 'YYYY-MM-DD') like $2", &[&id, &date]).unwrap();
        res
    }

    #[requires(self.is_connected(), "database should be connected")]
    #[requires(self.table_exist("public", "request_table"), "request_table should be created in the database")]
    #[requires(self.table_exist("public", "status_table"), "status_table should be created in the database")]
    #[ensures(ret.len() <= 1)]
    #[invariant(true)] 
    pub fn get_status_history(&mut self, uuid: &Uuid) -> Vec<Row> {
        let res = self.client.query("select st.uuid, st.status 
                                     from status_table st
                                     where st.uuid = $1", &[&uuid]).unwrap();
        res
    }

}
