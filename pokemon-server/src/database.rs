use crate::data_type::RequestOrder;
use contracts::*;
use postgres::{Client, NoTls};
// use sqlx::postgres::PgPoolOptions;

pub struct Database {
    client: Client,
}

impl Database {
    #[ensures(ret.is_connected() == true, "database is connected")]
    pub fn new() -> Self {
        let mut db = Database {
            client: Client::connect(
                // prefix: postgresql://postgres
                // pwd: nctusrs0915904265
                // ip: localhost
                // port: 5432
                // db: pokemon
                "postgresql://postgres:nctusrs0915904265@localhost:5432/pokemon",
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
                .batch_execute("CREATE TYPE orderstatus AS ENUM('Confirmed', 'Filled');")
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
            Ok(n) => {}
            Err(e) => {
                panic!("[Database][insert] Error: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::Database;
}
