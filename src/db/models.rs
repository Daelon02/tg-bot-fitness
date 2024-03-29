use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: Uuid,
    pub telegram_id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: String,
    pub height: Option<i32>,
    pub weight: Option<i32>,
    pub age: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::diet_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DietLists {
    pub id: Uuid,
    pub user_id: Uuid,
    pub diet_list: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::trainings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trainings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_trainings: Value,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Insertable, Clone, Default)]
#[diesel(table_name = crate::db::schema::sizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sizes {
    pub id: Uuid,
    pub user_id: Uuid,
    pub chest: i32,
    pub waist: i32,
    pub hips: i32,
    pub hand_biceps: i32,
    pub leg_biceps: i32,
    pub calf: i32,
}
