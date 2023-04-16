use std::fmt;

use actix_web::{web, delete};
use actix_web::{web::{
    Data,
    Json,
    Path,
    Query,
}, get, post, HttpResponse};
use serde::de::IntoDeserializer;
use serde::{Deserialize, de};
use uuid::Uuid;

use crate::models::cocktails::CocktailData;
use crate::{models::cocktails::Cocktail, repository::database::Database};

#[derive(Deserialize)]
pub struct GenerateQuery {
    #[serde(deserialize_with = "deserialize_stringified_list")]
    pub ingredients: Vec<String>,
}
// https://github.com/actix/actix-web/issues/1301#issuecomment-747403932
pub fn deserialize_stringified_list<'de, D, I>(deserializer: D) -> std::result::Result<Vec<I>, D::Error> 
where 
    D: de::Deserializer<'de>,
    I: de::DeserializeOwned,
{
    struct StringVecNames<I>(std::marker::PhantomData<I>);

    impl<'de, I> de::Visitor<'de> for StringVecNames<I>
    where I: de::DeserializeOwned {
        type Value = Vec<I>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E> 
        where 
            E: de::Error,
        {
            let mut names = vec![];
            for name in v.split(",") {
                let name = I::deserialize(name.into_deserializer())?;
                names.push(name);
            }
            Ok(names)
        }
    }

    deserializer.deserialize_any(StringVecNames(std::marker::PhantomData::<I>))
}

#[get("/cocktails")]
pub async fn get_cocktails(db: Data<Database>) -> HttpResponse {
    let todos = Cocktail::get_cocktails(&db);
    HttpResponse::Ok().json(todos)
}

#[post("/cocktails")]
#[tracing::instrument]
pub async fn create_cocktail(db: Data<Database>, new_cocktail: Json<CocktailData>) -> HttpResponse {
    let todo = Cocktail::create_cocktail(&db, new_cocktail.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/cocktails/{id}")]
pub async fn get_cocktail_by_id(db: Data<Database>, id: Path<Uuid>) -> HttpResponse {
    let cocktail = Cocktail::get_cocktail_by_id(&db, id.into_inner());
    match cocktail {
        Some(cocktail) => HttpResponse::Ok().json(cocktail),
        None => HttpResponse::NotFound().body("Todo not found")
    }
}

#[delete("/cocktails/{id}")]
pub async fn delete_cocktail_by_id(db: Data<Database>, id: Path<Uuid>) -> HttpResponse {
    let deleted = Cocktail::delete_cocktail(&db, id.into_inner());
    match deleted {
        Some(del) => HttpResponse::Ok().json(del),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

// generate cocktail endpoint
// steps to gen 
//
// 1. get all ingredients in db with similar or the same name
// 2. order by cocktail_id
// 3. order by count and percentage of ingredients in cocktail (if 4 out of 5 ingredients are in
//    ingredient list passed to genereate endpoint add cocktail to return list of cocktails)
// 4. If percentage of cocktail is completed is less than 25% or 1 ingredinet out of
//    the total required for the cocktail. Ask chat gpt for more.
#[get("/cocktails/generate")]
pub async fn generate_cocktails(db: Data<Database>, query: Query<GenerateQuery>) -> HttpResponse {
    println!("[cocktails] generate_cocktails");
    let cocktails = Cocktail::generate_cocktails(&db, &query.ingredients);
    match cocktails {
        Ok(c) => HttpResponse::Ok().json(c),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bartender")
            .service(get_cocktails)
            .service(create_cocktail)
            .service(generate_cocktails)
            .service(get_cocktail_by_id)
            .service(delete_cocktail_by_id)
    );
}
