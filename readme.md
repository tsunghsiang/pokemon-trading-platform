# Introduction

The side project is essentially an interview assignment from [FST network](https://www.twincn.com/item.aspx?no=50763592). It aims to craft a simple pokemon-card trading platform for those who would like to buy/sell their own cards and query transaction statuses.

Different from other interviewees, I decided to complete the project in [Rust](https://www.rust-lang.org/). The corresponding tests, containerization, api specifications and db schemas are involved in the following sections. 

# Prerequisites

Remember to be ready with the following requirements if you would like to run the project on your machine as well as on docker containers.

- [Rust Toolchain](https://www.rust-lang.org/learn/get-started)
- [Docker Desktop](https://www.docker.com/get-started)
- [Postgres Database 14.1](https://www.postgresql.org/download/)

# Restful API Specifications
POST /api/pokemon/card: buy/sell a card
GET /api/pokemon/trade/{card}: view the latest 50 trades on each kind of card
GET /api/pokemon/order/{id}: view the status of latest 50 orders of a specific trader
# Logical Architecture
# Trading Contraints
1. card = { Pikachu, Bulbasaur, Charmander, Squirtle }
2. 1.00 <= price if a card <= 10.00 USD
3. total 10K users
4. A trader can only buy/sell 1 card per order at one time
5. Order Processing: FIFO
6. Tx occurs when (B: buy price of an buy order, S: sell price of a sell order)
	- [BUY] S <= B && S is the lowest among all sell orders. Tx price is at S
	- [SELL] S <= B && B is the highest among all buy orders, Tx price is at B
7. Traders can view the status of their latest 50 orders.
8. Traders can view the latest 50 trades on each kind of cards.
9. If the sequence of orders is fixed, the results must be the same no matter how many times you execute the sequence.
   
# DB Schema

![pokemon database schema](./images/pokemon-db-schema.png)

|Table Name|request_table|status_table|trade_table|
|-|-|-|-|
|**Usage**|The table records whole requests from traders, which is updated when an order comes in.|The table stores the latest order status with corresponding uuid. Once status of an order is updated, so is the table.|The table contains records of filled orders. That is to say, when buy-side and sell-side have traded with each other, the table is updated as well.

Let's dig deeper into the columns of each table. The thing you should bear in mind is that all tables are correlated with specific `uuid`, which is an unique identifier of an order, so that you could query state of an order with it .

First of all, let's investigate columns of table `request_table`
|Column|uuid|tm|side|order_px|vol|card|trader_id|
|:-|-|-|-|-|-|-|-|
|**Type**|uuid|timestamp|side (enum)|double|integer|card|integer|
|**Description**|unique id of an order|order time|Buy/Sell|order price|order volume|card type|unique trader-specific id|

Secondly, `status_table` records status of orders when an order is confirmed or filled in matching process.
|Column|uuid|status|
|:-|-|-|
|**Type**|uuid|orderstatus (enum)|
|**Description**|unique id of an order|Confirmed/Filled|

Lastly, we adopt a `trade_table` to store all *traded transactions* for further history queries.
|Column|buy_uuid|sell_uuid|buy_side_id|sell_side_id|tx_price|tx_vol|card|
|:-|-|-|-|-|-|-|-|
|**Type**|uuid|uuid|integer|integer|double|integer|card (enum)|
|**Description**|unique id of buy-side user|unique id of sell-side user|buy-side trader id|sell-side trader id|traded price|traded quantity|Pikachu/Bulbasaur/Charmander/Squirtle|

# Unit Tests Report
# Configuration
# Run on Local Host
curl localhost:8080/api/pokemon/card -X POST -H "Content-type:application/json" -d @test-script/post.json
curl -X GET localhost:8080/api/pokemon/trade/:card
curl -X GET localhost:8080/api/pokemon/order/:id
# Run on Docker Container
# Todo List
- [x] Restful API
- [x] Relational database (PostgreSQL, MySQL, ...)
- [x] Containerize (Docker & Docker Compose) 
- [x] Graceful shutdown (server & client)
- [x] Testing
- [x] Multithreading
- [ ] OpenAPI (Swagger)
- [x] Set up configurations using environment variables
- [x] Refactoring
- [ ] Readme 
- [ ] Optimize docker activation speed