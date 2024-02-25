use crate::db::models::{DietLists, Sizes, Trainings, Users};
use crate::errors::Result;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use serde_json::Value;
use teloxide::prelude::UserId;
use uuid::Uuid;

pub struct Db {
    pub conn: PgConnection,
}

impl Db {
    pub fn new(database_url: &str) -> Self {
        let conn = PgConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Self { conn }
    }

    pub async fn insert_user(
        &mut self,
        name: &str,
        phone_number: &str,
        telegram_id: &Option<UserId>,
    ) -> Result<()> {
        let id = Uuid::new_v4();
        let new_user = Users {
            id,
            telegram_id: telegram_id.unwrap_or(UserId(0)).0 as i32,
            name: name.to_string(),
            email: None,
            phone_number: phone_number.to_string(),
            height: None,
            weight: None,
            age: None,
        };
        diesel::insert_into(crate::db::schema::users::table)
            .values(&new_user)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn add_email(&mut self, phone_number: &str, email: &str) -> Result<()> {
        let _ = diesel::update(crate::db::schema::users::table)
            .filter(crate::db::schema::users::phone_number.eq(phone_number.to_string()))
            .set(crate::db::schema::users::email.eq(email.to_string()))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn add_age(&mut self, phone_number: &str, age: i32) -> Result<()> {
        let _ = diesel::update(crate::db::schema::users::table)
            .filter(crate::db::schema::users::phone_number.eq(phone_number.to_string()))
            .set(crate::db::schema::users::age.eq(age))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn add_height_and_weight(
        &mut self,
        phone_number: &str,
        height: i32,
        weight: i32,
    ) -> Result<()> {
        let _ = diesel::update(crate::db::schema::users::table)
            .filter(crate::db::schema::users::phone_number.eq(phone_number.to_string()))
            .set((
                crate::db::schema::users::height.eq(height),
                crate::db::schema::users::weight.eq(weight),
            ))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_user(&mut self, phone_number: &str) -> Result<Users> {
        log::info!("Getting user with phone number {}", phone_number);
        let user = crate::db::schema::users::table
            .filter(crate::db::schema::users::phone_number.eq(phone_number))
            .get_result(&mut self.conn)?;
        Ok(user)
    }

    pub async fn if_user_exists(&mut self, phone_number: &Option<UserId>) -> Result<bool> {
        let user: Option<String> = crate::db::schema::users::table
            .select(crate::db::schema::users::phone_number)
            .filter(
                crate::db::schema::users::telegram_id
                    .eq(phone_number.unwrap_or(UserId(0)).0 as i32),
            )
            .first(&mut self.conn)
            .optional()?;
        Ok(user.is_some())
    }

    #[allow(dead_code)]
    pub async fn get_user_by_id(&mut self, id: Uuid) -> Result<Users> {
        let user = crate::db::schema::users::table
            .filter(crate::db::schema::users::id.eq(id))
            .first(&mut self.conn)?;
        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn delete_user(&mut self, phone_number: &str) -> Result<()> {
        diesel::delete(crate::db::schema::users::table)
            .filter(crate::db::schema::users::phone_number.eq(phone_number.to_string()))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn insert_training(
        &mut self,
        user_id: Uuid,
        gym_training: &str,
        status: String,
    ) -> Result<()> {
        log::info!("Inserting gym training for user {}", user_id);
        let id = Uuid::new_v4();
        let new_gym_training = Trainings {
            id,
            user_id,
            user_trainings: serde_json::to_value(gym_training)?,
            status,
            created_at: chrono::Utc::now(),
            updated_at: None,
        };
        diesel::insert_into(crate::db::schema::trainings::table)
            .values(&new_gym_training)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_training(&mut self, user_id: Uuid, status: String) -> Result<Trainings> {
        let home_training = crate::db::schema::trainings::table
            .filter(crate::db::schema::trainings::user_id.eq(user_id))
            .filter(crate::db::schema::trainings::status.eq(status))
            .first(&mut self.conn)?;
        Ok(home_training)
    }

    #[allow(dead_code)]
    pub async fn update_training(
        &mut self,
        user_id: Uuid,
        training: Value,
        status: String,
    ) -> Result<()> {
        let _ = diesel::update(crate::db::schema::trainings::table)
            .filter(crate::db::schema::trainings::user_id.eq(user_id))
            .filter(crate::db::schema::trainings::status.eq(status))
            .set(crate::db::schema::trainings::user_trainings.eq(training))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn delete_training(&mut self, user_id: Uuid, status: String) -> Result<()> {
        diesel::delete(crate::db::schema::trainings::table)
            .filter(crate::db::schema::trainings::user_id.eq(user_id))
            .filter(crate::db::schema::trainings::status.eq(status))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn insert_diet_list(&mut self, user_id: Uuid, diet_list: &str) -> Result<()> {
        let id = Uuid::new_v4();
        let new_diet_list = DietLists {
            id,
            user_id,
            diet_list: serde_json::to_value(diet_list)?,
            created_at: chrono::Utc::now(),
            updated_at: None,
        };
        diesel::insert_into(crate::db::schema::diet_lists::table)
            .values(&new_diet_list)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_diet_list(&mut self, user_id: Uuid) -> Result<DietLists> {
        let diet_list = crate::db::schema::diet_lists::table
            .filter(crate::db::schema::diet_lists::user_id.eq(user_id))
            .first(&mut self.conn)?;
        Ok(diet_list)
    }

    #[allow(dead_code)]
    pub async fn update_diet_list(&mut self, user_id: Uuid, diet_list: Value) -> Result<()> {
        let _ = diesel::update(crate::db::schema::diet_lists::table)
            .filter(crate::db::schema::diet_lists::user_id.eq(user_id))
            .set(crate::db::schema::diet_lists::diet_list.eq(diet_list))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn delete_diet_list(&mut self, user_id: Uuid) -> Result<()> {
        diesel::delete(crate::db::schema::diet_lists::table)
            .filter(crate::db::schema::diet_lists::user_id.eq(user_id))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn update_age(&mut self, user_id: Uuid, age: i32) -> Result<()> {
        let _ = diesel::update(crate::db::schema::users::table)
            .filter(crate::db::schema::users::id.eq(user_id))
            .set(crate::db::schema::users::age.eq(age))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn update_height_and_weight(
        &mut self,
        user_id: Uuid,
        height: i32,
        weight: i32,
    ) -> Result<()> {
        let _ = diesel::update(crate::db::schema::users::table)
            .filter(crate::db::schema::users::id.eq(user_id))
            .set((
                crate::db::schema::users::height.eq(height),
                crate::db::schema::users::weight.eq(weight),
            ))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn update_size(&mut self, user_id: Uuid, size: Vec<String>) -> Result<()> {
        let chest = size[0].parse()?;
        let waist = size[1].parse()?;
        let hips = size[2].parse()?;
        let hand_biceps = size[3].parse()?;
        let leg_biceps = size[4].parse()?;
        let calf = size[5].parse()?;

        let sizes = Sizes {
            id: Uuid::new_v4(),
            user_id,
            chest,
            waist,
            hips,
            hand_biceps,
            leg_biceps,
            calf,
        };
        let _ = diesel::insert_into(crate::db::schema::sizes::table)
            .values(&sizes)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_size_by_user(&mut self, user_id: Uuid) -> Result<Option<Sizes>> {
        let user = crate::db::schema::sizes::table
            .filter(crate::db::schema::sizes::user_id.eq(user_id))
            .first(&mut self.conn)
            .optional()?;
        Ok(user)
    }

    pub async fn get_sizes_by_user(&mut self, user_id: Uuid) -> Result<Option<Vec<Sizes>>> {
        let user = crate::db::schema::sizes::table
            .filter(crate::db::schema::sizes::user_id.eq(user_id))
            .load(&mut self.conn)
            .optional()?;
        Ok(user)
    }
}
