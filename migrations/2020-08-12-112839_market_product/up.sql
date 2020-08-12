-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE market_products (
  id SERIAL PRIMARY KEY,
  product_id VARCHAR(255) NOT NULL,
  category_id VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  published BOOLEAN NOT NULL DEFAULT TRUE,
  price VARCHAR(255) NOT NULL,
  avaliable_status BOOLEAN NOT NULL DEFAULT TRUE,
  avaliable_on VARCHAR(255) NOT NULL DEFAULT 'now',
  store_quantity VARCHAR(255) NOT NULL,
  store_location VARCHAR(255) NOT NULL,
  image TEXT NOT NULL,
  description Text NOT NUll,
  rating Int Not Null Default 4,
  temp_delete BOOLEAN NOT NULL DEFAULT FALSE,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);