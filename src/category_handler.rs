
// use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::PgConnection;
use rocket_contrib::json::{JsonValue};
use crate::models::{Category,NewCategory,UpdateCategory};
use crate::schema;

pub fn add_category(con:PgConnection,category:NewCategory)-> JsonValue {
    // print!("Product is {:?}",product);
    use schema::market_products_categories;
    diesel::insert_into(market_products_categories::table)
                                                .values(category)
                                                .execute(&con)
                                                .expect("Error creating new category");
    return json!({
                "status": true,
                "data":"Category added successfully"
            })

}

pub fn update_old_category(con:PgConnection,category:UpdateCategory) -> JsonValue {
    use schema::market_products_categories::dsl::*;

    let results = diesel::update(&category)
                                                .set((
                                                    title.eq(&category.title),
                                                    icon.eq(&category.icon),
                                                    update_at.eq(&category.update_at)
                                                ))
                                                .get_results::<Category>(&con)
                                                .expect("Error updating category");
    return json!({
                "status": true,
                "data":results
            })
}

pub fn get_avaliable_category(con:PgConnection) -> JsonValue {
    use schema::market_products_categories::dsl::*;
    let results = market_products_categories.load::<Category>(&con)
    .expect("Error loading avaliable category");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}

pub fn delete_category(con:PgConnection,cid:i32) -> JsonValue {
    use schema::market_products_categories::dsl::*;

    diesel::delete(market_products_categories.filter(id.eq(cid)))
     .execute(&con)
        .expect("Error deleting category");
    return json!({
            "status": true,
            "data":"Category deleted successfully"
        })
}



