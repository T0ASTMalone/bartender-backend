use actix_web::{web, delete};
use actix_web::{web::{
    Data,
    Json,
    Path,
}, get, post, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{models::users::{User, NewUser}, repository::database::Database};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub email: String,
}

#[get("/users")]
pub async fn get_users(db: Data<Database>) -> HttpResponse {
    let todos = User::get_users(&db);
    HttpResponse::Ok().json(todos)
}

#[post("/users")]
// #[tracing::instrument]
pub async fn create_user(db: Data<Database>, user_input: Json<InputUser>) -> HttpResponse {
    let todo = User::create_user(&db, NewUser {
        username: &user_input.username,
        email: &user_input.email,
    });
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(db: Data<Database>, id: Path<i32>) -> HttpResponse {
    let user = User::get_user_by_id(&db, id.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found")
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(db: Data<Database>, id: Path<i32>) -> HttpResponse {
    let deleted = User::delete_user(&db, id.into_inner());
    match deleted {
        Ok(del) => HttpResponse::Ok().json(del),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bartender")
            .service(get_users)
            .service(create_user)
            // TODO: update user
            .service(get_user_by_id)
            .service(delete_user_by_id)
    );
}
