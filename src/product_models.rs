use crate::schema::market_products;

use serde::{Serialize, Deserialize};

#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct Product {
    pub id:i32,
    pub product_id:String,
    pub category_id:String,
    pub title: String,
    pub published: bool,
    pub price: String,
    pub avaliable_status: String,
    pub store_quantity: String,
    pub store_location: String,
    pub temp_delete:bool,
    pub created_at: String,
    pub update_at: String
}

#[derive(Insertable,Debug)]
#[table_name="market_products"]
pub struct NewProduct {
    pub product_id:String,
    pub category_id:String,
    pub title: String,
    pub published: bool,
    pub price: String,
    pub avaliable_status: String,
    pub store_quantity: String,
    pub store_location: String,
    pub temp_delete:bool,
    pub created_at: String,
    pub update_at: String
} 
impl NewProduct {
   pub fn new(
    product_id: String, category_id: String,
    title: String, published: bool, 
    price: String, avaliable_status: String,
    store_quantity: String, store_location: String,
    temp_delete: bool, created_at: String,
    update_at: String,
    ) -> NewProduct {
        NewProduct {
             product_id,
             category_id,
             title,
             published,
             price,
             avaliable_status,
             store_quantity,
             store_location,
             temp_delete,
             created_at,
             update_at
        } 
    }
}

#[derive(Identifiable,Debug)]
#[table_name="market_products"]
pub struct UpdateProduct {
    pub id:i32,
    pub category_id:String,
    pub title: String,
    pub published: bool,
    pub price: String,
    pub avaliable_status: String,
    pub store_quantity: String,
    pub store_location: String,
    pub temp_delete:bool,
    pub update_at: String
} 
impl UpdateProduct {
   pub fn new(
    id:i32,
    category_id: String,
    title: String, 
    published: bool, 
    price: String, 
    avaliable_status: String,
    store_quantity: String,
    store_location: String,
    temp_delete: bool,
    update_at: String,

    ) -> UpdateProduct {
        UpdateProduct {
             id,
             category_id,
             title,
             published,
             price,
             avaliable_status,
             store_quantity,
             store_location,
             temp_delete,
             update_at
        } 
    }
}




#[derive(FromForm,Debug)]
    pub struct FormProduct {
        pub category_id:String,
        pub title: String,
        pub price: String,
        pub avaliable_status: String,
        pub store_quantity: String,
        pub store_location: String,
        pub published: bool,
    }
#[derive(FromForm,Debug)]
    pub struct UpdateForm {
        pub id:i32,
        pub product_id:String,
        pub category_id:String,
        pub title: String,
        pub published: bool,
        pub price: String,
        pub avaliable_status: String,
        pub store_quantity: String,
        pub store_location: String,
        pub temp_delete:bool,
        pub created_at: String,
        pub update_at: String
    } 