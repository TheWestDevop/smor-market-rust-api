use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
use chrono::prelude::*;
use uuid::Uuid;
use crate::product_handler;
use crate::order_handler;
use crate::models::{ NewOrder,UpdateOrder,OrderData,OrderUpdate};
use crate::auth::{
    NormalAdminApiKey,
    SuperAdminApiKey,
    UserApiKey
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