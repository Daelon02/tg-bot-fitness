CREATE TABLE users
(
    id           UUID PRIMARY KEY,
    telegram_id  INT NOT NULL PRIMARY KEY UNIQUE,
    name         VARCHAR NOT NULL,
    email        VARCHAR,
    phone_number VARCHAR NOT NULL
);