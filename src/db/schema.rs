diesel::table! {
    users (id) {
    id -> Uuid,
    telegram_id -> Integer,
    name -> Text,
    email -> Nullable<Text>,
    phone_number -> Text,
    height -> Nullable<Integer>,
    weight -> Nullable<Integer>,
    age -> Nullable<Integer>
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
    trainings_for_user (id) {
    id -> Uuid,
    user_id -> Uuid,
    trainings -> Jsonb,
    status -> Text,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
    }
}
