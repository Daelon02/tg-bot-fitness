use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: Uuid,
    pub telegram_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::diet_lists_for_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DietListForUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub diet_list: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::gym_trainings_for_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GymTrainingsForUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub gym_training: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::home_training_for_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HomeTrainingsForUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub home_training: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
