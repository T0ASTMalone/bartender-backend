use std::fmt::Error;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, RunQueryDsl, QueryDsl};

use crate::repository::schema::todos::dsl::*;
use crate::repository::schema::categories::dsl::*;

use crate::repository::database::Database;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::todos)]
pub struct Todo {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryData {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItemData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub category: Option<CategoryData>,
}

impl Todo {
    pub fn get_categories(db: &Database) -> Vec<Category> {
        categories
            .load::<Category>(&mut db.pool.get().unwrap())
            .expect("Error loading all categories")
    }

    pub fn get_todo_with_category(db: &Database) -> Vec<TodoItemData> {
        let mut empty_todo_item_data_list: Vec<TodoItemData> = Vec::new();

        todos
            .inner_join(categories)
            .load::<(Todo, Category)>(&mut db.pool.get().unwrap())
            .expect("Error loading all todos")
            .into_iter()
            .for_each(|(todo, category)| {
                let todo_item_data = TodoItemData { 
                    id: todo.id,
                    title: todo.title,
                    description: todo.description,
                    created_at: todo.created_at,
                    updated_at: todo.updated_at,
                    category: Some(CategoryData {
                        id: category.id,
                        name: category.name,
                        description: category.description
                    })
                };
                empty_todo_item_data_list.push(todo_item_data)
            });
        empty_todo_item_data_list
    }

    pub fn get_todos(db: &Database) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut db.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn create_todo(db: &Database, todo: Todo) -> Result<Todo, Error> {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..todo
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut db.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    pub fn get_todo_by_id(db: &Database, todo_id: &str) -> Option<Todo> {
        let todo = todos
            .find(todo_id)
            .get_result::<Todo>(&mut db.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(todo)
    }

    pub fn delete_todo_by_id(db: &Database, todo_id: &str) -> Option<usize> {
        let count = diesel::delete(todos.find(todo_id))
            .execute(&mut db.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }

    pub fn update_todo_by_id(db: &Database, todo_id: &str, mut todo: Todo) -> Option<Todo> {
        todo.updated_at = Some(Utc::now().naive_utc());
        let todo = diesel::update(todos.find(todo_id))
            .set(&todo)
            .get_result::<Todo>(&mut db.pool.get().unwrap())
            .expect("Error updating todo by id");
        Some(todo)
    }
}
