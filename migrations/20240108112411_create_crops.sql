-- Add migration script here
CREATE TABLE crops (
    id TEXT PRIMARY KEY,
    type_code TEXT,
    crop_code TEXT,
    crop_name TEXT
);
