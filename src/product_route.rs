use rocket_contrib::json::{JsonValue};
use rocket::request::{Form};
use chrono::prelude::*;
use uuid::Uuid;
use crate::product_handler;
use crate::store_handler;
use crate::models::{FormProduct,NewProduct,UpdateProduct,UpdateForm};
use crate::auth::{SuperAdminApiKey,NormalAdminApiKey,UserApiKey};


#[get("/")]
pub fn index() -> JsonValue {
    return json!({
        "status": "success",
        "message": "Hello, Welcome to smorfarm market web service"
    })
}
#[get("/avaliable/products")]
pub fn user_avaliable_products(_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::get_user_avaliable_products(connect);
    
}
#[get("/all/stores/unavaliable/products")]
pub fn all_stores_unavaliable_products(_auth:SuperAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::get_admin_unavaliable_products(connect);
   
}

#[get("/all/stores/temp/delete/products")]
pub fn all_stores_temp_delete_products(_auth:SuperAdminApiKey) -> JsonValue {
    
     let connect = product_handler::establish_connection();
    
     return product_handler::get_admin_all_temp_delete_products(connect)
   
}

#[get("/avaliable/products/<store_keeper_id>")]
pub fn avaliable_products(store_keeper_id:String,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    let store_location = store_handler::get_store_location(store_keeper_id,connect);
    if store_location == "none"{
        return json!({
            "status": false,
            "message": "No Store Assign To User, Contact Super Admin"
        })
    }else{
        return product_handler::get_avaliable_products(store_location,product_handler::establish_connection());
    }
    
}

#[get("/unavaliable/products/<store_keeper_id>")]
pub fn unavaliable_products(store_keeper_id:String,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    let store_location = store_handler::get_store_location(store_keeper_id,connect);
    if store_location == "none"{
        return json!({
            "status": false,
            "message": "No Store Assign To User, Contact Super Admin"
        })
    }else{
    return product_handler::get_unavaliable_products(store_location,product_handler::establish_connection());
   }
}

#[get("/all/temp/delete/products/<store_keeper_id>")]
pub fn all_temp_delete_products(store_keeper_id:String,_auth:NormalAdminApiKey) -> JsonValue {
    
     let connect = product_handler::establish_connection();
    let store_location = store_handler::get_store_location(store_keeper_id,connect);
    if store_location == "none"{
        return json!({
            "status": false,
            "message": "No Store Assign To User, Contact Super Admin"
        })
    }else{
     return product_handler::get_all_temp_delete_products(store_location,product_handler::establish_connection())
   }
}

 

#[get("/product/category/<id>")]
pub fn products_by_category(id:String,_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::product_by_category(connect,id);
}

#[get("/product/search?<product>")]
pub fn search_product(product:String,_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::get_product(connect,product);
}

#[get("/product/<category>/search?<product>")]
pub fn search_product_by_category(category:String,product:String,_auth:UserApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::get_product_by_category(connect,category,product);
}

#[get("/product/<id>")]
pub fn get_product(id:String,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::get_single_product(connect,id);
}

#[post("/add/product", data = "<item>")]
pub fn add_new_product(item:Form<FormProduct>,_auth:NormalAdminApiKey) -> JsonValue {

    let time  = Local::now();
    let p_id = Uuid::new_v5(
        &Uuid::NAMESPACE_OID,
        time.to_string().as_bytes()
    );

    let new_product =  NewProduct::new(
        p_id.to_string(), 
        item.category_id.to_string(), 
        item.title.to_string(), 
        item.published, 
        item.price.to_string(), 
        item.avaliable_status,
        item.avaliable_on.to_string(),
        item.store_quantity.to_string(), 
        item.store_location.to_string(), 
        false, 
        time.to_string(), 
        time.to_string(),
        item.product_images.to_string(),
        item.description.to_string(),
    );

    // print!("New Product is {:?}",new_product);


    let connect = product_handler::establish_connection();

    return product_handler::add_product(connect,new_product);

}

#[put("/update/product", data = "<item>")]
pub fn update_product(item:Form<UpdateForm>,_auth:NormalAdminApiKey) -> JsonValue {

    let time  = Local::now();

    let update_product =  UpdateProduct::new(
        item.id,
        item.category_id.to_string(), 
        item.title.to_string(),
        item.published, 
        item.price.to_string(), 
        item.avaliable_status,
        item.avaliable_on.to_string(),
        item.store_quantity.to_string(), 
        item.store_location.to_string(), 
        item.product_images.to_string(), 
        item.description.to_string(),
        item.temp_delete, 
        time.to_string(),
    );

    // print!("New Product is {:?}",new_product);


    let connect = product_handler::establish_connection();

    return product_handler::update_old_product(connect,update_product);
}

#[patch("/temp/product/delete/state", data = "<item>")]
pub fn temp_delete_product(item:Form<UpdateForm>,_auth:NormalAdminApiKey) -> JsonValue {

    let time  = Local::now();

    let update_product =  UpdateProduct::new(
        item.id,
        item.category_id.to_string(), 
        item.title.to_string(), 
        item.published, 
        item.price.to_string(), 
        item.avaliable_status, 
        item.avaliable_on.to_string(),
        item.store_quantity.to_string(), 
        item.store_location.to_string(), 
        item.product_images.to_string(),
        time.to_string(),
        item.temp_delete, 
        time.to_string()
    );

    let connect = product_handler::establish_connection();

    return product_handler::temp_delete_and_undelete_product(connect,update_product);

}

#[delete("/delete/product/<id>")]
pub fn permanent_delete_product(id:String,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = product_handler::establish_connection();
    return product_handler::delete_product(connect,id);
}

//error middleware

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": false,
        "message": "Nothing found."
    })
}
#[catch(401)]
pub fn not_authorised() -> JsonValue {
    json!({
        "status": false,
        "message": "The request requires authentication."
    })
}
#[catch(203)]
pub fn not_authoritative() -> JsonValue {
    json!({
        "status": false,
        "message": "non-authoritative token given."
    })
}
#[catch(500)]
pub fn server_error() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like we messed up."
    })
}
#[catch(400)]
pub fn bad_request() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you send a bad request."
    })
}
#[catch(403)]
pub fn forbidden_request() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you are forbidden from accessing this service, contact admin."
    })
}
#[catch(422)]
pub fn unprocessable_entity() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you send a bad request."
    })
}

