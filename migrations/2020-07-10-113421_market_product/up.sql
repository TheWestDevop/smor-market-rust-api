-- Your SQL goes here
ALTER TABLE market_products
  ADD product_images TEXT;

ALTER TABLE market_products_orders ALTER COLUMN coupon SET NOT NULL;