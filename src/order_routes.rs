use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
use chrono::prelude::*;
use uuid::Uuid;
use crate::product_handler;
use crate::order_handler;
use crate::models::{ NewOrder,UpdateOrder,OrderData,OrderUpdate,CouponData,NewCoupon};
use crate::auth::{
    NormalAdminApiKey,
    UserApiKey,
    SuperAdminApiKey
};


#[get("/all/orders")]
pub fn all_orders(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::get_all_orders(connect);
}
#[get("/all/<id>/orders")]
pub fn all_user_orders(id:String,_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::get_all_user_orders(connect,id);
}
#[get("/all/pre/orders")]
pub fn all_pre_orders(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::get_all_pre_orders(connect);
}
#[get("/all/normal/orders")]
pub fn all_normal_orders(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::get_all_normal_orders(connect);
}
#[get("/all/pending/pre/orders")]
pub fn all_pending_pre_orders(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::get_all_pending_pre_orders(connect);
}
#[get("/all/pending/normal/orders")]
pub fn all_pending_normal_orders(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::get_all_pending_normal_orders(connect);
}

#[post("/make/order", data = "<order>")]
pub fn make_order(order:Form<OrderData>, _auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    let time  = Local::now();
    let order_id = Uuid::new_v5(
        &Uuid::NAMESPACE_OID,
        time.to_string().as_bytes()
    );
   let order =  NewOrder::new(
       order_id.to_string(), 
       order.user_id.to_string(), 
       order.delivery_address.to_string(), 
       order.product_ordered.to_string(),
       order.total_cost.to_string(), 
       order.coupon.to_string(), 
       order.order_type, 
       1, 
       time.to_string(), 
       time.to_string() );
    return order_handler::add_to_order(connect,order);
}

#[put("/update/order/status", data = "<order>")]
pub fn update_status(order:Form<OrderUpdate>,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    let time  = Local::now();
    
    let data =
     UpdateOrder::new(
         order.id, 
         order.order_id.to_string(), 
         order.user_id.to_string(), 
         order.delivery_address.to_string(), 
         order.product_ordered.to_string(), 
         order.total_cost.to_string(), 
         order.coupon.clone(), 
         order.order_type, 
         order.order_status, 
         time.to_string()
         );
    return order_handler::update_order_status(connect, data)
}

#[put("/create/coupon", data = "<coupon>")]
pub fn create_coupon(coupon:Form<CouponData>,_auth:SuperAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    let data = NewCoupon::new(coupon.coupon.to_string(), coupon.amount.to_string());
    return order_handler::add_coupon(connect, data);
}
#[patch("/coupon/use/<coupon>")]
pub fn use_coupon(coupon:String,_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::coupon_used(connect,coupon);
}
#[get("/all/coupon")]
pub fn all_coupons(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::all_coupon(connect);
}
#[get("/all/used/coupon")]
pub fn all_used_coupons(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::all_used_coupon(connect);
}
#[get("/all/unused/coupon")]
pub fn all_unused_coupons(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::all_unused_coupon(connect);
}
#[delete("/delete/coupon/<coupon>")]
pub fn remove_coupon(coupon:i32,_auth:SuperAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return order_handler::delete_coupon(connect,coupon);
}

