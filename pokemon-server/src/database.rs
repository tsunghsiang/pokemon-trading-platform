use postgres::{Client, NoTls};

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Self {
        Database {
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
        }
    }

    pub fn init_tables(&mut self) {
        // create enum 'Side'
        if !self.enum_type_exist("Side") {
            self.client
                .batch_execute("CREATE TYPE Side AS ENUM('Buy', 'Sell')")
                .unwrap();
        }

        // create enum 'Card'
        if !self.enum_type_exist("Card") {
            self.client
                .batch_execute(
                    "CREATE TYPE Card AS ENUM('Pikachu', 'Bulbasaur', 'Charmander', 'Squirtle')",
                )
                .unwrap();
        }

        // create enum 'OrderStatus'
        if !self.enum_type_exist("OrderStatus") {
            self.client
                .batch_execute("CREATE TYPE OrderStatus AS ENUM('Confirmed', 'Filled')")
                .unwrap();
        }

        // create table 'request_table'
        self.client
            .batch_execute(
                "create table if not exists request_table(
                    uuid UUID,
                    tm timestamptz,
                    side Side,
                    order_px double,
                    vol INTEGER,
                    card Card,
                    trader_id, INTEGER 
                )",
            )
            .unwrap();

        // create table 'status_table'
        self.client
            .batch_execute(
                "create table if not exists status_table(
                    uuid UUID,
                    status OrderStatus
                )",
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
                    tx_price double,
                    tx_vol INTEGER,
                    card Card
                )",
            )
            .unwrap();
    }

    pub fn is_connected(&self) -> bool {
        !self.client.is_closed()
    }

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
}

#[cfg(test)]
mod tests {
    use crate::database::Database;

    #[test]
    fn given_database_instantiated_then_db_connected() {
        let db = Database::new();
        assert_eq!(true, db.is_connected());
    }
}
