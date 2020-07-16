-- Your SQL goes here
CREATE TABLE market_products_coupons (
  id SERIAL PRIMARY KEY,
  coupon VARCHAR(255) NOT NULL,
  amount VARCHAR(255) NOT NULL,
  coupon_use_status Boolean NOT NULL  DEFAULT false,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);