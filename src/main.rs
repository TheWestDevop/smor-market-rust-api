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
extern crate lettre_email;






mod schema;

mod product_handler;
mod product_route;
mod category_handler;
mod category_routes;
mod order_handler;
mod order_routes;
mod store_handler;
mod store_routes;
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
        order_routes::use_coupon,
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
        order_routes::all_coupons,
        order_routes::all_unused_coupons,
        order_routes::all_used_coupons,
        order_routes::create_coupon,
        order_routes::remove_coupon,
        order_routes::all_pre_orders,
        order_routes::all_normal_orders,
        order_routes::all_pending_pre_orders,
        order_routes::all_pending_normal_orders,
        store_routes::avaliable_store,
        store_routes::add_new_store,
        store_routes::update_store,
        store_routes::delete_store
        ])
    .register(
        catchers![
            product_route::not_found,
            product_route::server_error,
            product_route::bad_request,
            product_route::unprocessable_entity,
            product_route::not_authorised,
            product_route::not_authoritative,
            product_route::forbidden_request
        ]
    )
    .launch();
}
