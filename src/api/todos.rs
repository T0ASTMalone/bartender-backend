use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
    Path,
}, get, post, put, delete,  HttpResponse};

use crate::{models::todo::Todo, repository::database::Database};

#[get("")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = Todo::get_todos(&db);
    HttpResponse::Ok().json(todos) 
}

#[post("")]
// #[tracing::instrument]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = Todo::create_todo(&db, new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_todo_by_id(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let todo = Todo::get_todo_by_id(&db, &id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found")
    }
}

#[put("/{id}")]
pub async fn update_todo_by_id(
    db: Data<Database>,
    id: Path<String>, 
    updated_todo: Json<Todo>
) -> HttpResponse {
    let todo = Todo::update_todo_by_id(&db, &id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/{id}")]
pub async fn delete_todo_by_id(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let deleted = Todo::delete_todo_by_id(&db, &id);
    match deleted {
        Some(del) => HttpResponse::Ok().json(del),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(get_todos)
            .service(create_todo)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(delete_todo_by_id)
    );
}
