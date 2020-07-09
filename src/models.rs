use crate::schema::*;

use serde::{Serialize, Deserialize};
//  use jsonwebtoken::{ decode, Validation,DecodingKey,Algorithm};
use rocket::{Outcome, request::{self,Request,FromRequest}, http::Status};
// use jwt
use hmac::{Hmac, NewMac};
use jwt::*;
use sha2::Sha256;
use std::collections::BTreeMap;
use rocket_contrib::json::{JsonValue};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`


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
    #[derive(Queryable,Serialize, Deserialize,Debug)]
    pub struct Category {
        pub id:i32,
        pub title: String,
        pub details: String,
        pub created_at: String,
        pub update_at: String
    }
    
    #[derive(Insertable,Debug)]
    #[table_name="market_products_categories"]
    pub struct NewCategory {
        pub title: String,
        pub details: String,
        pub created_at: String,
        pub update_at: String
    } 
    impl NewCategory {
       pub fn new(
        title: String,
        details: String, 
        created_at: String,
        update_at: String,
        ) -> NewCategory {
            NewCategory {
                 title,
                 details,
                 created_at,
                 update_at
            } 
        }
    }
    
    #[derive(Identifiable,Debug)]
    #[table_name="market_products_categories"]
    pub struct UpdateCategory {
        pub id:i32,
        pub title: String,
        pub details: String,
        pub update_at: String
    } 
    impl UpdateCategory {
       pub fn new(
        id:i32,
        title: String,
        details: String, 
        update_at: String,
        ) -> UpdateCategory {
            UpdateCategory {
                 id,
                 title,
                 details,
                 update_at
            } 
        }
    }
    
    
    
    
    #[derive(FromForm,Debug)]
        pub struct CategoryData {
            pub title: String,
            pub details: String,
        }
    #[derive(FromForm,Debug)]
        pub struct CategoryUpdate {
            pub id:i32,
            pub title: String,
            pub details: String,
            pub created_at: String,
            pub update_at: String
        } 

    pub fn verify_token(token:&str) -> Result<String,String>{
        let key: Hmac<Sha256> = Hmac::new_varkey(b"mysecret").unwrap();
        let token_claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();
        // match claims {
        //  Ok(claims) => claims['']
        // }
        // print!("key length --> {:?}",token_claims.len());

       if token_claims.is_empty() {
        return Err("Token not valid".to_string());
       }else{
        return Ok(token_claims["sub"].to_string());
       }
    }
   pub fn generate_token() -> JsonValue {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"mysecret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "smor_admin");
    claims.insert("company", "smor_group");
    claims.insert("exp", "smor_admin");


    let token = claims.sign_with_key(&key).unwrap();
    if token.is_empty() {
        return json!({
            "status":"error",
            "message":"an error occurred kindly try again"
        });
       }else{
        return json!({
            "status":"success",
            "token":token
        });
    }
   }
#[derive(Debug)]
pub struct  ApiKey (pub String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()>{
      
        let keys: Vec<_> = request.headers().get("authorization").collect();


        // print!("request header ---> {:?}",keys);
        
         if keys.is_empty() || keys.len() != 1  || keys[0] == ""{ 
           return  Outcome::Failure((Status::Unauthorized,()));
         }
         match verify_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            
            Err(_) => Outcome::Forward(())
        }
        // return  keys;
    }
}