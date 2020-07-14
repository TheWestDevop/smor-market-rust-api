#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate uuid;
extern crate chrono;
extern crate jwt;
extern crate hmac;
extern crate sha2;






mod schema;

mod product_handler;
mod product_route;
mod category_handler;
mod category_routes;
mod order_handler;
mod order_routes;
mod models;
mod auth;
fn main() {

    product_handler::establish_connection();

    rocket::ignite()
    .mount("/api/v1", routes![
        product_route::index,
        product_route::avaliable_products,
        product_route::unavaliable_products,
        product_route::search_product,
        product_route::search_product_by_category,
        product_route::products_by_category,
        product_route::get_product,
        category_routes::avaliable_category,
        order_routes::make_order,
        order_routes::all_user_orders,
        ])
    .mount("/api/v1/admin", routes![
        product_route::index,
        product_route::add_new_product,
        product_route::update_product,
        product_route::temp_delete_product,
        product_route::all_temp_delete_products,
        product_route::permanent_delete_product,
        category_routes::add_new_category,
        category_routes::update_category,
        category_routes::delete_category,
        order_routes::update_status,
        order_routes::all_orders,
        
        ])
    .register(
        catchers![
            product_route::not_found,
            product_route::server_error,
            product_route::bad_request,
            product_route::unprocessable_entity,
            product_route::not_authorised,
            product_route::not_authoritative
        ]
    )
    .launch();
}
