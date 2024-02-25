CREATE TABLE trainings (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    trainings jsonb NOT NULL,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
