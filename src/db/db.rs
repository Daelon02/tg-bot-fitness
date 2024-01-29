use crate::db::models::{DietListForUser, GymTrainingsForUser, HomeTrainingsForUser, Users};
use crate::errors::{Errors, Result};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
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
            telegram_id: telegram_id.unwrap_or(UserId(0)).0 as i64,
            name: name.to_string(),
            email: None,
            phone_number: phone_number.to_string(),
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

    pub async fn get_user(&mut self, phone_number: &str) -> Result<Users> {
        let user = crate::db::schema::users::table
            .filter(crate::db::schema::users::phone_number.eq(phone_number.to_string()))
            .first(&mut self.conn)?;
        Ok(user)
    }

    pub async fn if_user_exists(&mut self, phone_number: &Option<UserId>) -> Result<bool> {
        let user: Option<Users> = crate::db::schema::users::table
            .filter(
                crate::db::schema::users::telegram_id
                    .eq(phone_number.unwrap_or(UserId(0)).0 as i64),
            )
            .first(&mut self.conn)
            .optional()?;
        Ok(user.is_some())
    }

    pub async fn get_user_by_id(&mut self, id: Uuid) -> Result<Users> {
        let user = crate::db::schema::users::table
            .filter(crate::db::schema::users::id.eq(id))
            .first(&mut self.conn)?;
        Ok(user)
    }

    pub async fn delete_user(&mut self, phone_number: &str) -> Result<()> {
        diesel::delete(crate::db::schema::users::table)
            .filter(crate::db::schema::users::phone_number.eq(phone_number.to_string()))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn insert_gym_training(&mut self, user_id: Uuid, gym_training: &str) -> Result<()> {
        let id = Uuid::new_v4();
        let new_gym_training = GymTrainingsForUser {
            id,
            user_id,
            gym_training: serde_json::to_value(gym_training)?,
            created_at: chrono::Utc::now(),
            updated_at: None,
        };
        diesel::insert_into(crate::db::schema::gym_trainings_for_user::table)
            .values(&new_gym_training)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_gym_training(&mut self, user_id: Uuid) -> Result<GymTrainingsForUser> {
        let gym_training = crate::db::schema::gym_trainings_for_user::table
            .filter(crate::db::schema::gym_trainings_for_user::user_id.eq(user_id))
            .first(&mut self.conn)?;
        Ok(gym_training)
    }

    pub async fn update_gym_training(&mut self, user_id: Uuid, gym_training: Value) -> Result<()> {
        let _ = diesel::update(crate::db::schema::gym_trainings_for_user::table)
            .filter(crate::db::schema::gym_trainings_for_user::user_id.eq(user_id))
            .set(crate::db::schema::gym_trainings_for_user::gym_training.eq(gym_training))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn delete_gym_training(&mut self, user_id: Uuid) -> Result<()> {
        diesel::delete(crate::db::schema::gym_trainings_for_user::table)
            .filter(crate::db::schema::gym_trainings_for_user::user_id.eq(user_id))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn insert_home_training(&mut self, user_id: Uuid, home_training: &str) -> Result<()> {
        let id = Uuid::new_v4();
        let new_home_training = HomeTrainingsForUser {
            id,
            user_id,
            home_training: serde_json::to_value(home_training)?,
            created_at: chrono::Utc::now(),
            updated_at: None,
        };
        diesel::insert_into(crate::db::schema::home_training_for_user::table)
            .values(new_home_training)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_home_training(&mut self, user_id: Uuid) -> Result<GymTrainingsForUser> {
        let home_training = crate::db::schema::home_training_for_user::table
            .filter(crate::db::schema::home_training_for_user::user_id.eq(user_id))
            .first(&mut self.conn)?;
        Ok(home_training)
    }

    pub async fn update_home_training(
        &mut self,
        user_id: Uuid,
        home_training: Value,
    ) -> Result<()> {
        let _ = diesel::update(crate::db::schema::home_training_for_user::table)
            .filter(crate::db::schema::home_training_for_user::user_id.eq(user_id))
            .set(crate::db::schema::home_training_for_user::home_training.eq(home_training))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn delete_home_training(&mut self, user_id: Uuid) -> Result<()> {
        diesel::delete(crate::db::schema::home_training_for_user::table)
            .filter(crate::db::schema::home_training_for_user::user_id.eq(user_id))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn insert_diet_list(&mut self, user_id: Uuid, diet_list: &str) -> Result<()> {
        let id = Uuid::new_v4();
        let new_diet_list = DietListForUser {
            id,
            user_id,
            diet_list: serde_json::to_value(diet_list)?,
            created_at: chrono::Utc::now(),
            updated_at: None,
        };
        diesel::insert_into(crate::db::schema::diet_lists_for_user::table)
            .values(&new_diet_list)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn get_diet_list(&mut self, user_id: Uuid) -> Result<GymTrainingsForUser> {
        let diet_list = crate::db::schema::diet_lists_for_user::table
            .filter(crate::db::schema::diet_lists_for_user::user_id.eq(user_id))
            .first(&mut self.conn)?;
        Ok(diet_list)
    }

    pub async fn update_diet_list(&mut self, user_id: Uuid, diet_list: Value) -> Result<()> {
        let _ = diesel::update(crate::db::schema::diet_lists_for_user::table)
            .filter(crate::db::schema::diet_lists_for_user::user_id.eq(user_id))
            .set(crate::db::schema::diet_lists_for_user::diet_list.eq(diet_list))
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub async fn delete_diet_list(&mut self, user_id: Uuid) -> Result<()> {
        diesel::delete(crate::db::schema::diet_lists_for_user::table)
            .filter(crate::db::schema::diet_lists_for_user::user_id.eq(user_id))
            .execute(&mut self.conn)?;
        Ok(())
    }
}
