use crate::schema::market_products_orders;

use serde::{Serialize, Deserialize};

#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct Order {
    pub id:i32,
    pub order_id: String,
    pub user_id: String,
    pub delivery_address: String,
    pub product_ordered: String,
    pub total_cost: String,
    pub order_type:i32,
    pub order_status:i32,
    pub coupon: String,
    pub created_at: String,
    pub update_at: String
}

#[derive(Insertable,Debug)]
#[table_name="market_products_orders"]
pub struct NewOrder {
    pub order_id: String,
    pub user_id: String,
    pub delivery_address: String,
    pub product_ordered: String,
    pub total_cost: String,
    pub coupon: String,
    pub order_type:i32,
    pub order_status:i32,
    pub created_at: String,
    pub update_at: String
} 
impl NewOrder {
   pub fn new(
    order_id: String,
    user_id: String, 
    delivery_address: String,
    product_ordered: String,
    total_cost: String,
    coupon: String,
    order_type:i32,
    order_status:i32,
    created_at: String,
    update_at: String,
    ) -> NewOrder {
        NewOrder {
             order_id,
             user_id,
             delivery_address,
             product_ordered,
             total_cost,
             coupon,
             order_type,
             order_status,
             created_at,
             update_at
        } 
    }
}

#[derive(Identifiable,Debug)]
#[table_name="market_products_orders"]
pub struct UpdateOrder {
    pub id:i32,
    pub order_id: String,
    pub user_id: String,
    pub delivery_address: String,
    pub product_ordered: String,
    pub total_cost: String,
    pub coupon: String,
    pub order_type:i32,
    pub order_status:i32,
    pub update_at: String
} 
impl UpdateOrder {
   pub fn new(
    id:i32,
    order_id: String,
    user_id: String, 
    delivery_address: String,
    product_ordered: String,
    total_cost: String,
    coupon: String,
    order_type:i32,
    order_status:i32,
    update_at: String,
    ) -> UpdateOrder {
        UpdateOrder {
             id,
             order_id,
             user_id,
             delivery_address,
             product_ordered,
             total_cost,
             coupon,
             order_type,
             order_status,
             update_at
        } 
    }
}



#[derive(FromForm,Debug)]
    pub struct OrderData {
        pub user_id: String,
        pub delivery_address: String,
        pub product_ordered: String,
        pub total_cost: String,
        pub coupon: String,
        pub order_type:i32,
    }
#[derive(FromForm,Debug)]
    pub struct OrderUpdate {
        pub id:i32,
        pub order_id: String,
        pub user_id: String,
        pub delivery_address: String,
        pub product_ordered: String,
        pub total_cost: String,
        pub coupon: String,
        pub order_type:i32,
        pub order_status:i32,
        pub created_at: String,
        pub update_at: String
    } 