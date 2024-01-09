-- Add migration script here
CREATE TABLE crops_summary_14_days (
    id TEXT PRIMARY KEY,
    end_date TEXT,
    crop_code TEXT,
    high_price_beta_coefficient REAL,
    mid_price_beta_coefficient REAL,
    low_price_beta_coefficient REAL,
    average_price_beta_coefficient REAL,
    trading_volume_beta_coefficient REAL
    trading_volume_sum REAL
);
