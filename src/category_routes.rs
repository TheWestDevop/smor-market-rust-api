use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
use chrono::prelude::*;
use crate::product_handler;
use crate::category_handler;
use crate::models::{NewCategory,UpdateCategory,CategoryData,CategoryUpdate};
use crate::auth::*;


#[get("/categories")]
pub fn avaliable_category(_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return category_handler::get_avaliable_category(connect);
}

#[post("/add/category", data = "<item>")]
pub fn add_new_category(item:Form<CategoryData>,_auth:NormalAdminApiKey) -> JsonValue {
    let time  = Local::now();
    let new_category =  NewCategory::new(
        item.icon.to_string(),  
        item.title.to_string(), 
        time.to_string(), 
        time.to_string()
    );

    let connect = product_handler::establish_connection();

    return category_handler::add_category(connect,new_category);

}

#[put("/update/category", data = "<item>")]
pub fn update_category(item:Form<CategoryUpdate>,_auth:NormalAdminApiKey) -> JsonValue {

    let time  = Local::now();

    let update_product =  UpdateCategory::new(
        item.id, 
        item.icon.to_string(), 
        item.title.to_string(), 
        time.to_string()
    );

    let connect = product_handler::establish_connection();

    return category_handler::update_old_category(connect,update_product);
}

#[delete("/delete/category/<id>")]
pub fn delete_category(id:i32,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return category_handler::delete_category(connect,id);
}