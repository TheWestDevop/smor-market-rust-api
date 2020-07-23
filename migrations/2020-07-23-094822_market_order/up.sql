-- Your SQL goes here
CREATE TABLE market_products_orders (
  id SERIAL PRIMARY KEY,
  order_id VARCHAR(255) NOT NULL,
  user_id VARCHAR(255) NOT NULL,
  delivery_state VARCHAR(255) NOT NULL,
  delivery_lga VARCHAR(255) NOT NULL,
  delivery_address VARCHAR(255) NOT NULL,
  delivery_at VARCHAR(255) NOT NULL DEFAULT 'Today',
  product_ordered TEXT NOT NULL,
  total_cost VARCHAR(255) NOT NULL,
  order_type INT NOT NULL,
  order_status INT NOT NULL,
  coupon VARCHAR(255) NOT NUll,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);