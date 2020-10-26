-- Your SQL goes here
CREATE TABLE market_store (
  id SERIAL PRIMARY KEY,
  staff_id VARCHAR(255) NOT NULL,
  store_keeper VARCHAR(255) NOT NULL,
  store_location VARCHAR(255) NOT NULL, 
  store_address VARCHAR(255) NOT NULL,
  active_status BOOLEAN NOT NULL DEFAULT TRUE,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);