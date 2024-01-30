CREATE TABLE users
(
    id           UUID PRIMARY KEY,
    telegram_id  INT NOT NULL,
    name         VARCHAR NOT NULL,
    email        VARCHAR,
    phone_number VARCHAR NOT NULL,
    height       INT,
    weight       INT,
    age INT
);