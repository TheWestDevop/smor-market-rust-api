use diesel::prelude::*;
use diesel::PgConnection;
use rocket_contrib::json::{JsonValue};
use crate::models::{Order,NewOrder,UpdateOrder,NewCoupon,Coupon};
use crate::schema;


pub fn add_to_order(con:PgConnection,order:NewOrder) -> JsonValue {
    use schema::market_products_orders;
    diesel::insert_into(market_products_orders::table)
                                                .values(order)
                                                .execute(&con)
                                                .expect("Error submitting new order");
    return json!({
                "status": true,
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
                "status": true,
                "data":"order status update successfully."
            })
}

pub fn get_all_orders(con:PgConnection) -> JsonValue {
    use schema::market_products_orders::dsl::*;
    
    let results = market_products_orders.load::<Order>(&con)
    .expect("Error loading orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn get_all_pre_orders(con:PgConnection) -> JsonValue {
    use schema::market_products_orders::dsl::*;
    
    let results = market_products_orders.filter(order_type.eq(2)).load::<Order>(&con)
    .expect("Error loading orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn get_all_normal_orders(con:PgConnection) -> JsonValue {
    use schema::market_products_orders::dsl::*;
    
    let results = market_products_orders.filter(order_type.eq(1)).load::<Order>(&con)
    .expect("Error loading orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn get_all_pending_pre_orders(con:PgConnection) -> JsonValue {
    use schema::market_products_orders::dsl::*;
    
    let results = market_products_orders.filter(order_type.eq(2).and(order_status.eq(1).or(order_status.eq(2)))).load::<Order>(&con)
    .expect("Error loading pending pre orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn get_all_pending_normal_orders(con:PgConnection) -> JsonValue {
    use schema::market_products_orders::dsl::*;
    
    let results = market_products_orders.filter(order_type.eq(1).and(order_status.eq(1).or(order_status.eq(2)))).load::<Order>(&con)
    .expect("Error loading pending normal orders");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
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
        "status": true,
        "data":results
    })

}


pub fn add_coupon(con:PgConnection,coupon:NewCoupon)-> JsonValue {
    use schema::market_products_coupons;
    diesel::insert_into(market_products_coupons::table)
                                                .values(coupon)
                                                .execute(&con)
                                                .expect("Error submitting new coupon");
    return json!({
                "status": true,
                "data":"Coupon Added successfuly."
            })
}
pub fn all_coupon(con:PgConnection)-> JsonValue{
    use schema::market_products_coupons::dsl::*;
    
    let results = market_products_coupons.load::<Coupon>(&con)
    .expect("Error loading coupons");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn all_used_coupon(con:PgConnection)-> JsonValue{
    use schema::market_products_coupons::dsl::*;

    let results = market_products_coupons.filter(coupon_use_status.eq(true)).load::<Coupon>(&con)
    .expect("Error loading coupons");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn all_unused_coupon(con:PgConnection)-> JsonValue{
    use schema::market_products_coupons::dsl::*;

    let results = market_products_coupons.filter(coupon_use_status.eq(false)).load::<Coupon>(&con)
    .expect("Error loading coupons");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn coupon_use(con:PgConnection,cid:String)-> JsonValue{
    use schema::market_products_coupons::dsl::*;

   let coupon_status = diesel::update(market_products_coupons.filter(coupon.eq(cid).and(coupon_use_status.eq(false))))
                                                .set(
                                                    coupon_use_status.eq(true)
                                                )
                                                .get_result::<Coupon>(&con)
                                                .expect("Error updating coupon status");
    if coupon_status.coupon.eq("") {
        return json!({
            "status": true,
            "data":coupon_status
        })
    }else {
        return json!({
            "status": false,
            "data":"coupon has been used."
        })
    }
    
}
pub fn delete_coupon(con:PgConnection,cid:i32)-> JsonValue{
    use schema::market_products_coupons::dsl::*;

    diesel::delete(market_products_coupons.filter(id.eq(cid)))
    .execute(&con)
        .expect("Error deleting coupon");
    return json!({
            "status": true,
            "data":"Coupon deleted successfully"
        })
}
