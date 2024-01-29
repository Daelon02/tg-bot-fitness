CREATE TABLE gym_trainings_for_user (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    gym_training jsonb NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
