use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
    Path,
}, get, post, put, delete,  HttpResponse};

use crate::{models::todo::Todo, repository::database::Database};

#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos) 
}

#[post("/todos")]
#[tracing::instrument]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let todo = db.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found")
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(
    db: Data<Database>,
    id: Path<String>, 
    updated_todo: Json<Todo>
) -> HttpResponse {
    let todo = db.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let deleted = db.delete_todo_by_id(&id);
    match deleted {
        Some(del) => HttpResponse::Ok().json(del),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_todos)
            .service(create_todo)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(delete_todo_by_id)
    );
}
