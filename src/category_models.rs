use crate::schema::market_products_categories;

use serde::{Serialize, Deserialize};

#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct Category {
    pub id:i32,
    pub title: String,
    pub details: String,
    pub created_at: String,
    pub update_at: String
}

#[derive(Insertable,Debug)]
#[table_name="market_products_categories"]
pub struct NewCategory {
    pub title: String,
    pub details: String,
    pub created_at: String,
    pub update_at: String
} 
impl NewCategory {
   pub fn new(
    title: String,
    details: String, 
    created_at: String,
    update_at: String,
    ) -> NewCategory {
        NewCategory {
             title,
             details,
             created_at,
             update_at
        } 
    }
}

#[derive(Identifiable,Debug)]
#[table_name="market_products_categories"]
pub struct UpdateCategory {
    pub id:i32,
    pub title: String,
    pub details: String,
    pub update_at: String
} 
impl UpdateCategory {
   pub fn new(
    id:i32,
    title: String,
    details: String, 
    update_at: String,
    ) -> UpdateCategory {
        UpdateCategory {
             id,
             title,
             details,
             update_at
        } 
    }
}




#[derive(FromForm,Debug)]
    pub struct CategoryData {
        pub title: String,
        pub details: String,
    }
#[derive(FromForm,Debug)]
    pub struct CategoryUpdate {
        pub id:i32,
        pub title: String,
        pub details: String,
        pub created_at: String,
        pub update_at: String
    } 