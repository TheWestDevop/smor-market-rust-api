use diesel::prelude::*;
use diesel::PgConnection;
use rocket_contrib::json::{JsonValue};
use crate::models::{Store,NewStore,UpdateStore};
use crate::schema;

pub fn add_store(con:PgConnection,store:NewStore)-> JsonValue {
    // print!("Product is {:?}",product);
    use schema::market_store;
    let result = diesel::insert_into(market_store::table)
                                                .values(store)
                                                .get_result::<Store>(&con)
                                                .expect("Error creating new store");
    return json!({
                "status": true,
                "data":result
            })

}
pub fn update_old_store(con:PgConnection,store:UpdateStore) -> JsonValue {
    use schema::market_store::dsl::*;

    let results = diesel::update(&store)
                                                .set((
                                                    staff_id.eq(&store.staff_id),
                                                    store_keeper.eq(&store.store_keeper),
                                                    store_location.eq(&store.store_location),
                                                    store_address.eq(&store.store_address),
                                                    active_status.eq(&store.active_status),
                                                    update_at.eq(&store.update_at)
                                                ))
                                                .get_results::<Store>(&con)
                                                .expect("Error updating category");
    return json!({
                "status": true,
                "data":results
            })
}
pub fn get_avaliable_store(con:PgConnection) -> JsonValue {
    use schema::market_store::dsl::*;
    let results = market_store.load::<Store>(&con)
    .expect("Error loading avaliable store");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn delete_store(con:PgConnection,sid:i32) -> JsonValue {
    use schema::market_store::dsl::*;

    diesel::delete(market_store.filter(id.eq(sid)))
     .execute(&con)
        .expect("Error deleting store");
    return json!({
            "status": true,
            "data":"Store deleted successfully"
        })
}

pub fn get_store_location(keeper_id:String,con:PgConnection) -> String {
    use schema::market_store::dsl::*;

    let result = market_store.filter(staff_id.eq(&keeper_id)).load::<Store>(&con)
    .expect("Error loading avaliable store");
     if result.len().eq(&0){
         return "none".to_string();
     }else{
        let store = &result[0].store_location;
        return store.to_string();
     }
}