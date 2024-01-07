-- Add migration script here
CREATE TABLE daily_crop_transactions (
    id TEXT PRIMARY KEY,
    transaction_date TEXT,
    type_code TEXT,
    crop_code TEXT,
    crop_name TEXT,
    high_price REAL,
    mid_price REAL,
    low_price REAL,
    average_price REAL,
    trading_volume REAL
);
