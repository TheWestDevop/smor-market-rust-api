-- Your SQL goes here
CREATE TABLE market_products (
  id SERIAL PRIMARY KEY,
  product_id VARCHAR(255) NOT NULL,
  category_id VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  published BOOLEAN NOT NULL DEFAULT TRUE,
  price VARCHAR(255) NOT NULL,
  avaliable_status VARCHAR(255) NOT NULL,
  store_quantity VARCHAR(255) NOT NULL,
  store_location VARCHAR(255) NOT NULL,
  product_image TEXT NOT NULL,
  temp_delete BOOLEAN NOT NULL DEFAULT FALSE,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);