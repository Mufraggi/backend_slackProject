CREATE TABLE city (
                      id SERIAL PRIMARY KEY,
                      name VARCHAR NOT NULL,
                      gps_point VARCHAR NOT NULL,
                      mail VARCHAR NOT NULL,
                      phone_number VARCHAR NOT NULL,
                      validate BOOLEAN NOT NULL DEFAULT 'f'
);
-- Your SQL goes here