use diesel::dsl::count;
use diesel::expression::ValidGrouping;
// use diesel::pg::Pg;
use diesel::result::Error;
use diesel::{Queryable, Insertable, RunQueryDsl, Selectable, Identifiable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::schema::ingredients::dsl::*;
use crate::repository::database::Database;

#[derive(Queryable, Selectable, Identifiable, Debug, Deserialize, Insertable, ValidGrouping)]
#[diesel(table_name = crate::repository::schema::ingredients)]
pub struct Ingredient {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub measurement: String,
    pub cocktail_id: Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IngredientData {
    pub id: Option<Uuid>,
    pub name: String,
    pub measurement: String,
    pub cocktail_id: Option<Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<(String, String)> for IngredientData {
    fn from (tuple: (String, String)) -> Self {
        Self {
            id: None,
            name: tuple.0,
            measurement: tuple.1,
            cocktail_id: None,
            updated_at: None,
            created_at: None,
        }
    }
}

impl From<Ingredient> for IngredientData {
    fn from(ingredient: Ingredient) -> Self {
        Self {
           id: Some(ingredient.id),
           name: ingredient.name,
           measurement: ingredient.measurement,
           cocktail_id: Some(ingredient.cocktail_id),
           created_at: ingredient.created_at,
           updated_at: ingredient.updated_at,
        }
    }
}

impl From<IngredientData> for Ingredient {
    fn from(ins: IngredientData) -> Self {
        Self {
           id: ins.id.unwrap_or(uuid::Uuid::new_v4()),
           name: ins.name,
           measurement: ins.measurement,
           cocktail_id: ins.cocktail_id.unwrap(),
           created_at: ins.created_at,
           updated_at: ins.updated_at,
        }

    }
}

impl Ingredient { 
    pub fn create_ingredient(db: &Database, ingredient: Ingredient) {
        diesel::insert_into(ingredients)
            .values(&ingredient)
            .execute(&mut db.pool.get().unwrap())
            .expect("Error inserting ingredient");
    }

    pub fn create_ingredients(db: &Database, ing_vec: Vec<Ingredient>) -> Result<usize, Error> {
        diesel::insert_into(ingredients)
            .values(&ing_vec)
            .execute(&mut db.pool.get().unwrap())
    }

    pub fn get_ingredients_by_cocktail_id(db: &Database, c_id: &Uuid) -> Result<Vec<Ingredient>, Error> {
        ingredients.filter(cocktail_id.eq(c_id))
            .get_results::<Ingredient>(&mut db.pool.get().unwrap())
    }

    // TODO: update to use From trait
    pub fn map_ingredient_to_data(ins: &Ingredient) -> IngredientData {
       IngredientData {
           id: Some(ins.id),
           name: ins.name.clone(),
           measurement: ins.measurement.clone(),
           cocktail_id: Some(ins.cocktail_id),
           created_at: ins.created_at,
           updated_at: ins.updated_at,
       }
    }

    pub fn map_ingredients_to_data(ins: Vec<Ingredient>) -> Vec<IngredientData> {
       ins.iter().map(|x| Ingredient::map_ingredient_to_data(x)).collect() 
    }

    // TODO: update to use From trait
    pub fn map_data_to_ingredient(ins: &IngredientData, c_id: &Uuid) -> Ingredient {
       Ingredient {
           id: ins.id.unwrap_or(uuid::Uuid::new_v4()),
           name: ins.name.clone(),
           measurement: ins.measurement.clone(),
           cocktail_id: ins.cocktail_id.unwrap_or(c_id.clone()),
           created_at: ins.created_at,
           updated_at: ins.updated_at,
       }
    }

    pub fn map_data_to_ingredients(ins: Vec<IngredientData>, c_id: &Uuid) -> Vec<Ingredient> {
       ins.iter().map(|x| Ingredient::map_data_to_ingredient(x, c_id)).collect() 
    }


    // TODO: pass in the desired min ingredient count 
    pub fn get_ingredients_by_names(db: &Database, ns: &Vec<String>) -> Result<Vec<Ingredient>, Error> {
        // 2. order by cocktail_id
        // 3. order by count and percentage of ingredients in cocktail (if 4 out of 5 ingredients are in
        //    ingredient list passed to genereate endpoint add cocktail to return list of cocktails)
        // 4. If percentage of cocktail is completed is less than 25% or 1 ingredinet out of
        let query = ingredients.filter(name.eq_any(ns))
            .select(cocktail_id)
            .group_by(cocktail_id)
            // so that the user could limit the min number of ingredients 
            // in the suggested cocktails
            // TODO: update this to be a variable that is passed in
            .having(count(cocktail_id).ge(1))
            // is this necessary if we don't need to use the count?
            // TODO: remove if not needed
            .order(count(cocktail_id).desc());

        // println!("{}", debug_query::<Pg, _>(&query));
        let ids = query.get_results::<Uuid>(&mut db.pool.get().unwrap())?;

        // will this preserve the order of the last query? Probabbly not
        // TODO: look into sorting the result
        ingredients.filter(cocktail_id.eq_any(ids))
            .get_results::<Ingredient>(&mut db.pool.get().unwrap())
    }
}
