CREATE TABLE diet_lists
(
    id         UUID PRIMARY KEY,
    user_id    UUID      NOT NULL,
    diet_list  jsonb     NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
