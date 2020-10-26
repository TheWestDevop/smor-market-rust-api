use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
// use chrono::prelude::*;
use crate::product_handler;
use crate::store_handler;
use crate::models::{NewStore,UpdateStore,StoreData,StoreUpdate};
use crate::auth::*;

#[get("/stores")]
pub fn avaliable_store(_auth:SuperAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return store_handler::get_avaliable_store(connect);
}

#[post("/add/store", data = "<item>")]
pub fn add_new_store(item:Form<StoreData>,_auth:SuperAdminApiKey) -> JsonValue {
    let new_store =  NewStore::new(
        item.staff_id.to_string(),  
        item.store_keeper.to_string(), 
        item.store_location.to_string(),
        item.store_address.to_string(), 
    );

    let connect = product_handler::establish_connection();

    return store_handler::add_store(connect,new_store);

}

#[put("/update/store", data = "<item>")]
pub fn update_store(item:Form<StoreUpdate>,_auth:SuperAdminApiKey) -> JsonValue {

    let update =  UpdateStore::new(
        item.id, 
        item.staff_id.to_string(),  
        item.store_keeper.to_string(), 
        item.store_location.to_string(),
        item.store_address.to_string(), 
        item.active_status
    );

    let connect = product_handler::establish_connection();

    return store_handler::update_old_store(connect,update);
}

#[delete("/delete/store/<id>")]
pub fn delete_store(id:i32,_auth:SuperAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return store_handler::delete_store(connect,id);
}