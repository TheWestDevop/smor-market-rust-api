use dotenv::dotenv;
use std::env;
// use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::PgConnection;
use rocket_contrib::json::{JsonValue};
use crate::product_models::{Product,NewProduct,UpdateProduct};
use crate::schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database. \n Database url is required!!! .");
    let db_connection = PgConnection::establish(&database_url).expect("error connecting to database.");
    return db_connection  
}

pub fn add_product(con:PgConnection,product:NewProduct)-> JsonValue {
    // print!("Product is {:?}",product);
    use schema::market_products;
    let results = diesel::insert_into(market_products::table)
                                                .values(product)
                                                .get_result::<Product>(&con)
                                                .expect("Error creating new product");
    return json!({
                "status": "success",
                "data":results
            })

 }

pub fn update_old_product(con:PgConnection,product:UpdateProduct) -> JsonValue {
    use schema::market_products::dsl::*;

    let results = diesel::update(&product)
                                                .set((
                                                    title.eq(&product.title),
                                                    category_id.eq(&product.category_id),
                                                    published.eq(&product.published),
                                                    price.eq(&product.price),
                                                    avaliable_status.eq(&product.avaliable_status),
                                                    store_quantity.eq(&product.store_quantity),
                                                    store_location.eq(&product.store_location),
                                                    temp_delete.eq(&product.temp_delete),
                                                    update_at.eq(&product.update_at)
                                                ))
                                                .get_result::<Product>(&con)
                                                .expect("Error updating product");
    return json!({
                "status": "success",
                "data":results
            })
}

pub fn temp_delete_and_undelete_product(con:PgConnection,product:UpdateProduct) -> JsonValue{
    use schema::market_products::dsl::*;
    diesel::update(&product)
                                                .set((
                                                    temp_delete.eq(&product.temp_delete),
                                                    update_at.eq(&product.update_at)
                                                ))
                                                .execute(&con)
                                                .expect("Error updating product delete state");
    return json!({
                "status": "success",
                "data":"Product state changed succesfully"
            })
}

pub fn delete_product(con:PgConnection,pid:String) -> JsonValue {
    use schema::market_products::dsl::*;
    diesel::delete(market_products.filter(product_id.eq(pid)))
    .execute(&con)
        .expect("Error deleting category");
    return json!({
            "status": "success",
            "data":"Product deleted successfully"
        })
}

pub fn get_avaliable_products(con:PgConnection) -> JsonValue {
    use schema::market_products::dsl::*;
    let results = market_products.filter(published.eq(true).and(temp_delete.eq(false)))
    .load::<Product>(&con)
    .expect("Error loading avaliable products");
    // print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
 }

pub fn get_unavaliable_products(con:PgConnection)-> JsonValue{
    use schema::market_products::dsl::*;
    let results = market_products.filter(published.eq(false).and(temp_delete.eq(false)))
    .load::<Product>(&con)
    .expect("Error loading unavaliable products");
    // print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
 }

pub fn product_by_category(con:PgConnection,cate_id:String) -> JsonValue {
    use schema::market_products::dsl::*;
    let results = market_products.filter(category_id.eq(cate_id).and(temp_delete.eq(false)))
    .load::<Product>(&con)
    .expect("Error loading products by category");
    // print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
 }

pub fn get_product(con:PgConnection,query:String) -> JsonValue {
    use schema::market_products::dsl::*;
    let results = market_products.filter(title.ilike(query).and(temp_delete.eq(false)))
    .load::<Product>(&con)
    .expect("Error loading searched products");
    print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
 }

pub fn get_product_by_category(con:PgConnection,cate_id:String, query:String) -> JsonValue {
    use schema::market_products::dsl::*;
    let results = market_products.filter(category_id.eq(cate_id).and(title.ilike(query)).and(temp_delete.eq(false)))
    .load::<Product>(&con)
    .expect("Error loading searched products");
    print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
 }

pub fn get_all_temp_delete_products(con:PgConnection) -> JsonValue {
    use schema::market_products::dsl::*;
    let results = market_products.filter(temp_delete.eq(false))
    .load::<Product>(&con)
    .expect("Error loading avaliable products");
    // print!("query result  {:?}",results);
    return json!({
        "status": "success",
        "data":results
    })
 }