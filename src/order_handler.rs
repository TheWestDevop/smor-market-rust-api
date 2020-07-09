use diesel::prelude::*;
use diesel::PgConnection;
use rocket_contrib::json::{JsonValue};
use crate::models::{Order,NewOrder,UpdateOrder};
use crate::schema;


pub fn add_to_order(con:PgConnection,order:NewOrder) -> JsonValue {
    use schema::market_products_orders;
    diesel::insert_into(market_products_orders::table)
                                                .values(order)
                                                .execute(&con)
                                                .expect("Error submitting new order");
    return json!({
                "status": "success",
                "data":"Order Submitted successfuly."
            })
}

pub fn update_order_status(con:PgConnection,order:UpdateOrder) -> JsonValue {
    use schema::market_products_orders::dsl::*;

    diesel::update(&order)
                                                .set((
                                                    order_status.eq(&order.order_status),
                                                    update_at.eq(&order.update_at)
                                                ))
                                                .execute(&con)
                                                .expect("Error updating product");
    return json!({
                "status": "success",
                "data":"order status update successfully."
            })
}

pub fn get_all_orders(con:PgConnection) -> JsonValue {
    use schema::market_products_orders::dsl::*;
    
    let results = market_products_orders.load::<Order>(&con)
    .expect("Error loading orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
}

pub fn get_all_user_orders(con:PgConnection,uid:String) -> JsonValue{
    use schema::market_products_orders::dsl::*;
    let results = market_products_orders.filter(user_id.eq(uid))
    .load::<Order>(&con)
    .expect("Error loading user orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })

}