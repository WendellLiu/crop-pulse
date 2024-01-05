-- Add migration script here
CREATE TABLE crop_transactions (
    transaction_date TEXT,
    type_code TEXT,
    crop_code TEXT,
    crop_name TEXT,
    market_code TEXT,
    market_name TEXT,
    high_price REAL,
    mid_price REAL,
    low_price REAL,
    average_price REAL,
    trading_volume REAL,
    id TEXT PRIMARY KEY
);

