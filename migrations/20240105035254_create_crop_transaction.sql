-- Add migration script here
CREATE TABLE crop_transactions (
  id INTEGER PRIMARY KEY,
  crop_name TEXT,
  quantity INTEGER,
  price REAL,
  transaction_date TEXT
);
