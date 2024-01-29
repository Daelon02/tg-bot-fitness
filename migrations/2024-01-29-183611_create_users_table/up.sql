CREATE TABLE users {
    id UUID PRIMARY KEY,
    telegram_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(255) NOT NULL,
    }