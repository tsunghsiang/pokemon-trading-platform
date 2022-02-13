Restful API Specification: (JSON)

OrderStatus {
    Ordered,
    Filled
}

RequestOrder {
    tm: timestamp,
    type: BUY/SELL,
    order_px: f64,
    card: { Pikachu/Bulbasaur/Charmander/Squirtle },
    trader_id: i32 (unique)    
}

OrderReport {
    tm: timestamp,
    type: BUY/SELL,
    order_status: { ordered, filled },
    order_px: f64,
    card_type: { Pikachu/Bulbasaur/Charmander/Squirtle },
    trader_id: i32 (unique)
}

FillReport {
    tm: timestamp,
    order_status: { ordered, filled },
    trade_px: f64,
    card_type: { Pikachu/Bulbasaur/Charmander/Squirtle }
    buy_side: i32 (unique)
    sell_side: i32 (unique)
}

POST /api/pokemon/card: buy/sell a card
GET /api/pokemon/trade/:card: view the latest 50 trades on each kind of card
GET /api/pokemon/order/:id: view the status of latest 50 orders of a specific trader

curl localhost:8080/api/pokemon/card -X POST -H "Content-type:application/json" -d @test-script/post.json
curl -X GET localhost:8080/api/pokemon/trade/:card
curl -X GET localhost:8080/api/pokemon/order/:id

Constraints:
1. card = { Pikachu, Bulbasaur, Charmander, Squirtle }
2. 1.00 <= price if a card <= 10.00 USD
3. total 10K users
4. A trader can only buy/sell 1 card per order at one time
5. Order Processing: FIFO
6. Tx occurs when (B: buy price of an buy order, S: sell price of a sell order)
   1. [BUY] S <= B && S is the lowest among all sell orders. Tx price is at S
   2. [SELL] S <= B && B is the highest among all buy orders, Tx proce is at B 
7. Traders can view the status of their latest 50 orders.
8. Traders can view the latest 50 trades on each kind of cards.
9. If the sequence of orders is fixed, the results must be the same no matter how many times you execute the sequence.

## Basic Requirements:
- RESTful API (v)
- Relational database (PostgreSQL, MySQL, ...) (v)
- Containerize (Docker)
- Testing (v)
- Gracefully shutdown (server & client)
## Advanced Requirements:
- Multithreading
- Maximize performance of finishing 1M orders
- OpenAPI (Swagger)
- Set up configs using environment variables
- Docker Compose
- Cloud computing platforms (AWS, Azure, GCP, ...) 
- CI/CD
- User authentication and authorization

todo list:
- run on docker (rustapp, postgresql db)
    * https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
    * https://hub.docker.com/_/postgres
    * https://myapollo.com.tw/zh-tw/bash-script-wait-for-it/
    * https://hub.docker.com/_/rust
    * https://medium.com/it-dead-inside/docker-containers-and-localhost-cannot-assign-requested-address-6ac7bc0d042b
    * https://ithelp.ithome.com.tw/articles/10239305
- refactoring (GET -> return json, not string | scheduler)
- config initialization
- graceful shutdown
- user authentication process
- OpenAPI (Swagger)
- readme & release plan

docker cmd:
docker network create pokemon-net | docker network create -d bridge pokemon-net
docker run --network pokemon-net -e POSTGRES_PASSWORD=test -e POSTGRES_DB=pokemon -d postgres
# /pokemon-trading-platform/pokemon-server
docker build -t pokemon-server .
docker run -p 8080:8080 --network pokemon-net --rm --name pokemon-server -d pokemon-server (problem with db conn)