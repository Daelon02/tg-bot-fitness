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
    diet_lists (id) {
    id -> Uuid,
    user_id -> Uuid,
    diet_list -> Jsonb,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
    }
}

diesel::table! {
    trainings (id) {
    id -> Uuid,
    user_id -> Uuid,
    user_trainings -> Jsonb,
    status -> Text,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
    }
}

diesel::table! {
    sizes (id) {
    id -> Uuid,
    user_id -> Uuid,
    chest -> Integer,
    waist -> Integer,
    hips -> Integer,
    hand_biceps -> Integer,
    leg_biceps -> Integer,
    calf -> Integer,
    }
}
