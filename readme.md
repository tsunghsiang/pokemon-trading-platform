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
GET /api/pokemon/{card}/trade: view the latest 50 trades on each kind of card
GET /api/pokemon/{id}/order: view the status of latest 50 orders of a specific trader

curl localhost:8080/api/pokemon/card -X POST -H "Content-type:application/json" -d @test-script/post.json
curl -X GET localhost:8080/api/pokemon/:card/trade
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
- RESTful API
- Relational database (PostgreSQL, MySQL, ...)
- Containerize (Docker)
- Testing
- Gracefully shutdown
## Advanced Requirements:
- Multithreading
- Maximize performance of finishing 1M orders
- OpenAPI (Swagger)
- Set up configs using environment variables
- Docker Compose
- Cloud computing platforms (AWS, Azure, GCP, ...) 
- CI/CD
- User authentication and authorization