table! {
    market_products (id) {
        id -> Int4,
        product_id -> Varchar,
        category_id -> Varchar,
        title -> Varchar,
        published -> Bool,
        price -> Varchar,
        avaliable_status -> Bool,
        avaliable_on -> Varchar,
        store_quantity -> Varchar,
        store_location -> Varchar,
        image -> Text,
        description -> Text,
        rating -> Int4,
        temp_delete -> Bool,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_products_categories (id) {
        id -> Int4,
        icon -> Varchar,
        title -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_products_coupons (id) {
        id -> Int4,
        coupon -> Varchar,
        amount -> Varchar,
        coupon_use_status -> Bool,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_products_orders (id) {
        id -> Int4,
        order_id -> Varchar,
        user_id -> Varchar,
        delivery_state -> Varchar,
        delivery_lga -> Varchar,
        delivery_address -> Varchar,
        delivery_at -> Varchar,
        product_ordered -> Text,
        total_cost -> Varchar,
        order_type -> Int4,
        order_status -> Int4,
        coupon -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    market_store (id) {
        id -> Int4,
        staff_id -> Varchar,
        store_keeper -> Varchar,
        store_location -> Varchar,
        store_address -> Varchar,
        active_status -> Bool,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    market_products,
    market_products_categories,
    market_products_coupons,
    market_products_orders,
    market_store,
);
