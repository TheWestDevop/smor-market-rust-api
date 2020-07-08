table! {
    market_products (id) {
        id -> Int4,
        product_id -> Varchar,
        category_id -> Varchar,
        title -> Varchar,
        published -> Bool,
        price -> Varchar,
        avaliable_status -> Varchar,
        store_quantity -> Varchar,
        store_location -> Varchar,
        temp_delete -> Bool,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_products_categories (id) {
        id -> Int4,
        title -> Varchar,
        details -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_products_coupons (id) {
        id -> Int4,
        coupon -> Varchar,
        amount -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_products_orders (id) {
        id -> Int4,
        order_id -> Varchar,
        user_id -> Varchar,
        delivery_address -> Varchar,
        product_ordered -> Text,
        total_cost -> Varchar,
        order_type -> Int4,
        order_status -> Int4,
        coupon -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    market_products,
    market_products_categories,
    market_products_coupons,
    market_products_orders,
);
