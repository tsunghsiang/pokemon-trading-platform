use data_type::RequestOrder;
use tide::Request;

mod data_type;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();
    server.at("/api/pokemon/card").post(user_request);
    server.at("/api/pokemon/trade/:card").get(view_card_status);
    server.at("/api/pokemon/order/:id").get(view_order_status);
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn user_request(mut req: Request<()>) -> tide::Result {
    let order: RequestOrder = req.body_json().await?;
    Ok(format!("Confirmed: {:?}", &order).into())
}

async fn view_card_status(req: Request<()>) -> tide::Result {
    let card = req.param("card").unwrap_or("None");
    Ok(format!("View the latest 50 trades on card: {}", &card).into())
}

async fn view_order_status(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap();
    Ok(format!("View the latest 50 orders of trader[{}]", &id).into())
}
