use serde::{Deserialize, Serialize};
use diesel::{prelude::*, Queryable, Insertable, AsChangeset, RunQueryDsl, QueryDsl, Selectable, Identifiable, result::Error};

use crate::repository::{schema::users::dsl::*, database::Database};

#[derive(Serialize, Selectable, Identifiable, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

impl User {
    pub fn get_users(db: &Database) -> Vec<User> {
        users.load::<User>(&mut db.pool.get().unwrap())
            .expect("Error loading users")
    }

    pub fn get_user_by_email(db: &Database, email_str: &str) -> Result<User, Error> {
        users.filter(email.eq(email_str)).get_result::<User>(&mut db.pool.get().unwrap())
    }

    pub fn get_user_by_id(db: &Database, user_id: i32) -> Result<User, Error> {
        users.find(user_id).get_result::<User>(&mut db.pool.get().unwrap())
    }

    pub fn create_user(db: &Database, new_user: NewUser) -> Result<User, Error> {
        diesel::insert_into(users).values(&new_user).get_result(&mut db.pool.get().unwrap())
    }

    pub fn delete_user(db: &Database, user_id: i32) -> Result<usize, Error> {
        diesel::delete(users.find(user_id)).execute(&mut db.pool.get().unwrap())
    }
}
