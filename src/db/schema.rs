diesel::table! {
    users (id) {
    id -> Uuid,
    telegram_id -> BigInt,
    name -> VarChar,
    email -> Nullable<VarChar>,
    phone_number -> VarChar
    }
}

diesel::table! {
    diet_lists_for_user (id) {
    id -> Uuid,
    user_id -> Uuid,
    diet_list -> Jsonb,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
    }
}

diesel::table! {
    home_training_for_user (id) {
    id -> Uuid,
    user_id -> Uuid,
    home_training -> Jsonb,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
    }
}

diesel::table! {
    gym_trainings_for_user (id) {
    id -> Uuid,
    user_id -> Uuid,
    gym_training -> Jsonb,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
    }
}
