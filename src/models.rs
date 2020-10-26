use crate::schema::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::Local;

//  use jsonwebtoken::{ decode, Validation,DecodingKey,Algorithm};
// use jwt
// use rocket_contrib::json::{JsonValue};
// use serde_json::value::Value;
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`


#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct Product {
    pub id:i32,
    pub product_id:String,
    pub category_id:String,
    pub title: String,
    pub published: bool,
    pub price: String,
    pub avaliable_status: bool,
    pub avaliable_on: String,
    pub store_quantity: String,
    pub store_location: String,
    pub image: String,
    pub description: String,
    pub rating:i32,
    pub temp_delete:bool,
    pub created_at: String,
    pub update_at: String,
}

#[derive(Insertable,Debug)]
#[table_name="market_products"]
pub struct NewProduct {
    pub product_id:String,
    pub category_id:String,
    pub title: String,
    pub published: bool,
    pub price: String,
    pub avaliable_status: bool,
    pub avaliable_on: String,
    pub store_quantity: String,
    pub store_location: String,
    pub image: String,
    pub description: String,
    pub temp_delete:bool,
    pub created_at: String,
    pub update_at: String,
} 
impl NewProduct {
   pub fn new(
    product_id: String, 
    category_id: String,
    title: String,
    published: bool, 
    price: String, 
    avaliable_status: bool,
    avaliable_on: String,
    store_quantity: String, 
    store_location: String,
    temp_delete: bool, 
    created_at: String,
    update_at: String,
    image: String,
    description: String,
    ) -> NewProduct {
        NewProduct {
             product_id,
             category_id,
             title,
             published,
             price,
             avaliable_status,
             avaliable_on,
             store_quantity,
             store_location,
             image,
             description,
             temp_delete,
             created_at,
             update_at,
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
    pub avaliable_status: bool,
    pub avaliable_on: String,
    pub store_quantity: String,
    pub store_location: String,
    pub image: String,
    pub description: String,
    pub temp_delete:bool,
    pub update_at: String,
} 
impl UpdateProduct {
   pub fn new(
    id:i32,
    category_id:String,
    title: String,
    published: bool,
    price: String,
    avaliable_status: bool,
    avaliable_on: String,
    store_quantity: String,
    store_location: String,
    image: String,
    description: String,
    temp_delete:bool,
    update_at: String,
    ) -> UpdateProduct {
        UpdateProduct {
            id,
            category_id,
            title,
            published,
            price,
            avaliable_status,
            avaliable_on,
            store_quantity,
            store_location,
            image,
            description,
            temp_delete,
            update_at,
        } 
    }
}




#[derive(FromForm,Debug)]
    pub struct FormProduct {
        pub category_id:String,
        pub title: String,
        pub product_images: String,
        pub price: String,
        pub avaliable_status: bool,
        pub avaliable_on: String,
        pub store_quantity: String,
        pub store_location: String,
        pub published: bool,
        pub description: String,
    }
#[derive(FromForm,Debug)]
    pub struct UpdateForm {
        pub id:i32,
        pub product_id:String,
        pub category_id:String,
        pub title: String,
        pub product_images: String,
        pub published: bool,
        pub price: String,
        pub avaliable_status: bool,
        pub avaliable_on: String,
        pub store_quantity: String,
        pub store_location: String,
        pub temp_delete:bool,
        pub created_at: String,
        pub update_at: String,
        pub description: String,
    } 

#[derive(Queryable,Serialize, Deserialize,Debug)]
    pub struct Order {
        pub id:i32,
        pub order_id: String,
        pub user_id: String,
        pub delivery_state: String,
        pub delivery_lga: String,
        pub delivery_address: String,
        pub delivery_at: String,
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
    pub delivery_state: String,
    pub delivery_lga: String,
    pub delivery_address: String,
    pub delivery_at: String,
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
    delivery_state: String,
    delivery_lga: String,
    delivery_address: String,
    delivery_at: String,
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
             delivery_state,
             delivery_lga,
             delivery_address,
             delivery_at,
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
    pub delivery_state: String,
    pub delivery_lga: String,
    pub delivery_address: String,
    pub delivery_at: String,
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
    delivery_state: String,
    delivery_lga: String,
    delivery_address: String,
    delivery_at: String,
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
             delivery_state,
             delivery_lga,
             delivery_address,
             delivery_at,
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
        pub delivery_state: String,
        pub delivery_lga: String,
        pub delivery_address: String,
        pub delivery_at: String,
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
        pub delivery_state: String,
        pub delivery_lga: String,
        pub delivery_address: String,
        pub delivery_at: String,
        pub product_ordered: String,
        pub total_cost: String,
        pub coupon: String,
        pub order_type:i32,
        pub order_status:i32,
        pub created_at: String,
        pub update_at: String
    } 
#[derive(Queryable,Serialize, Deserialize,Debug)]
    pub struct Category {
        pub id:i32,
        pub icon: String,
        pub title: String,
        pub created_at: String,
        pub update_at: String
    }
    
#[derive(Insertable,Debug)]
#[table_name="market_products_categories"]
    pub struct NewCategory {
        pub icon: String,
        pub title: String,
        pub created_at: String,
        pub update_at: String
    } 
    impl NewCategory {
       pub fn new(
        icon: String, 
        title: String,
        created_at: String,
        update_at: String,
        ) -> NewCategory {
            NewCategory {
                 icon,
                 title,
                 created_at,
                 update_at
            } 
        }
    }
    
    #[derive(Identifiable,Debug)]
    #[table_name="market_products_categories"]
    pub struct UpdateCategory {
        pub id:i32,
        pub icon: String,
        pub title: String,
        pub update_at: String
    } 
    impl UpdateCategory {
       pub fn new(
        id:i32,
        icon: String, 
        title: String,
        update_at: String,
        ) -> UpdateCategory {
            UpdateCategory {
                 id,
                 icon,
                 title,
                 update_at
            } 
        }
    }

    #[derive(Queryable,Serialize, Deserialize,Debug)]
   pub struct Coupon{
       pub id:i32,
       pub coupon:String,
       pub amount:String,
       pub coupon_use_status:bool,
       pub created_at:String,
       pub update_at:String
   } 

   #[derive(Insertable,Debug)]
   #[table_name="market_products_coupons"]
    pub struct NewCoupon{
    pub coupon:String,
    pub amount:String,
    pub created_at:String,
    pub update_at:String
} 
   impl NewCoupon {
       pub fn new(coupon:String,amount:String) -> NewCoupon {
          let  created_at = Local::now().to_string();
          let  update_at = Local::now().to_string();
            NewCoupon{
                coupon,
                amount,
                created_at,
                update_at
            }
       }
   }
    
    
    #[derive(FromForm,Debug)]
        pub struct CategoryData {
            pub title: String,
            pub icon: String,
        }
    #[derive(FromForm,Debug)]
        pub struct CategoryUpdate {
            pub id:i32,
            pub title: String,
            pub icon: String,
            pub created_at: String,
            pub update_at: String
        } 
        #[derive(FromForm,Debug)]
        pub struct CouponData {
            pub coupon: String,
            pub amount: String,
        }
        #[derive(FromForm,Debug)]
        pub struct StoreData {
            pub staff_id: String,
            pub store_keeper: String,
            pub store_location: String,
            pub store_address: String
        }

        #[derive(FromForm,Debug)]
        pub struct StoreUpdate {
            pub id:i32,
            pub staff_id: String,
            pub store_keeper: String,
            pub store_location: String,
            pub store_address: String,
            pub active_status: bool,
        }
        #[derive(Queryable,Serialize, Deserialize,Debug)]
        pub struct Store {
            pub id:i32,
            pub staff_id: String,
            pub store_keeper: String,
            pub store_location: String,
            pub store_address: String,
            pub active_status: bool,
            pub created_at: String,
            pub update_at: String
        }
        
    #[derive(Insertable,Debug)]
    #[table_name="market_store"]
        pub struct NewStore {
            pub staff_id: String,
            pub store_keeper: String,
            pub store_location: String,
            pub store_address: String,
            pub created_at: String,
            pub update_at: String
        } 
        impl NewStore {
           pub fn new(
            staff_id: String,
            store_keeper: String,
            store_location: String,
            store_address: String
            ) -> NewStore {
                let created_at = Local::now().to_string();
                let update_at = Local::now().to_string();
                let staff_id =  Uuid::new_v5(
             &Uuid::NAMESPACE_OID,
             format!("{}-{}-{}",staff_id,store_keeper,store_location).to_string().as_bytes()
         ).to_string();
                NewStore {
                    staff_id,
                    store_keeper,
                    store_location,
                    store_address,
                    created_at,
                    update_at,
                } 
            }
        }
        
        #[derive(Identifiable,Debug)]
        #[table_name="market_store"]
        pub struct UpdateStore {
            pub id:i32,
            pub staff_id: String,
            pub store_keeper: String,
            pub store_location: String,
            pub store_address: String,
            pub active_status: bool,
            pub update_at: String
        } 
        impl UpdateStore {
           pub fn new(
            id:i32,
            staff_id: String,
            store_keeper: String,
            store_location: String,
            store_address: String,
            active_status: bool,
            ) -> UpdateStore {
                let update_at = Local::now().to_string();
                UpdateStore {
                     id,
                     staff_id,
                     store_keeper,
                     store_location,
                     store_address,
                     active_status,
                     update_at
                } 
            }
        }
    







