
use chrono::Utc;
use diesel::result::Error;
use openai_api_rust::completions::{CompletionsBody, CompletionsApi};
use openai_api_rust::{Auth, OpenAI};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
// Note: prelude is required to use things like column.eq_any(vec)
use diesel::{prelude::*, Queryable, Insertable, AsChangeset, RunQueryDsl, QueryDsl, Selectable, Identifiable};

use crate::api::cocktails::GenerateQuery;
use crate::repository::schema::cocktails::dsl::*;
use crate::repository::schema::cocktails::columns::id;
use crate::repository::database::Database;

use super::ingredients::{Ingredient, IngredientData};
use super::instructions::{Instruction, InstructionData};


#[derive(Serialize, Selectable, Identifiable, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::cocktails)]
pub struct Cocktail {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailData {
    pub id: Option<Uuid>,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub instructions: Vec<InstructionData>,
    pub ingredients: Vec<IngredientData>,
}

impl From<(String, Vec<(String, String)>, Vec<String>)> for CocktailData {
    fn from(msg: (String, Vec<(String, String)>, Vec<String>)) -> Self {
        Self {
            id: None,
            name: msg.0.clone(),
            instructions: msg.2.iter()
                .enumerate()
                .map(|(i, x)| InstructionData::from((i, x.to_owned())))
                .collect(),
            ingredients: msg.1.iter()
                .map(|x| IngredientData::from(x.to_owned()))
                .collect(),
            created_at: None,
            updated_at: None,
        }
    }
}

impl CocktailData {
    pub fn parse_message(message: &str) -> Vec<CocktailData> {
        serde_json::from_str::<Vec<(String,Vec<(String, String)>, Vec<String>)>>(message).unwrap()
            .iter()
            .map(|message| { CocktailData::from(message.to_owned()) })
            .collect()
    }
}

impl Cocktail {
    pub fn get_cocktails(db: &Database) -> Vec<Cocktail> {
        cocktails
            .load::<Cocktail>(&mut db.pool.get().unwrap())
            .expect("Error loading all cocktails")
    }

    pub fn get_cocktail_by_id(db: &Database, c_id: Uuid) -> Option<CocktailData> {
        let mut conn = db.pool.get().unwrap();
        let cocktail = cocktails.find(&c_id)
            .get_result::<Cocktail>(&mut conn)
            .expect("Error loading cocktail by id");

        let ingredients = Ingredient::get_ingredients_by_cocktail_id(db, &c_id).unwrap();
        // TODO: fix me
        // let mapped_ings = ingredients.iter().map(|x| IngredientData::from(x)).collect();
        let mapped_ings = Ingredient::map_ingredients_to_data(ingredients);
        
        let instructions = Instruction::get_instructions_by_cocktail_id(db, &c_id).unwrap();
        // TODO: fix me
        // let mapped_ins = instructions.iter().map(|x| InstructionData::from(x.clone())).collect();
        let mapped_ins = Instruction::map_instructions_to_data(instructions);


        Some(CocktailData { 
            id: Some(cocktail.id), 
            name: cocktail.name,
            created_at: cocktail.created_at,
            updated_at: cocktail.updated_at,
            ingredients: mapped_ings, 
            instructions: mapped_ins, 
        })
    }
    
    pub fn create_cocktail(db: &Database, new_cocktail: CocktailData) -> Result<Cocktail, Error> {
        let mut con = db.pool.get().unwrap();

        let cocktail = Cocktail {
            id: uuid::Uuid::new_v4(),
            name: new_cocktail.name,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        };

        let cocktail_ingredients = Ingredient::map_data_to_ingredients(
            new_cocktail.ingredients, 
            &cocktail.id
        );
        let cocktail_instructions = Instruction::map_data_to_instructions(
            new_cocktail.instructions, 
            &cocktail.id
        );

        diesel::insert_into(cocktails)
            .values(&cocktail)
            .execute(&mut con) 
            .expect("Error creating new cocktail");

        let ingredients_insert = Ingredient::create_ingredients(db, cocktail_ingredients);

        match ingredients_insert { 
            Err(msg) => {
                Cocktail::delete_cocktail(db, cocktail.id);
                return Err(msg);
            },
            _ => (),
        };

        let instructions_insert = Instruction::create_instructions(db, cocktail_instructions);

        match instructions_insert { 
            Err(msg) => {
                Cocktail::delete_cocktail(db, cocktail.id);
                return Err(msg);
            },
            _ => (),
        };

        Ok(cocktail)
    }

    pub fn delete_cocktail(db: &Database, c_id: Uuid) -> Option<usize> {
        let count = diesel::delete(cocktails.find(c_id))
            .execute(&mut db.pool.get().unwrap())
            .expect("Error deleting cocktail");

        Some(count)
    }

    pub fn update_cocktail(db: &Database, cocktail: Cocktail) -> Option<Cocktail> {
        let updated_cocktail = diesel::update(cocktails.find(&cocktail.id))
            .set(&cocktail)
            .get_result::<Cocktail>(&mut db.pool.get().unwrap())
            .expect("Error updating cocktail");

        Some(updated_cocktail)
    }
    
    pub fn ask_gpt_for_cocktails(ingredients: &Vec<String>) -> Vec<CocktailData> {
        let ingredients_str = ingredients.join(",");
        let proompt = format!("What cockatils can I make with these ingredients? [{},ice] Format the result as a json array '[[name,[ingredents list],[instructions list]],...]'. Where 'name' is the cocktail name string, 'ingredients list' is an array of ingredient name and measurement tuples, and instructions is an array of instruction strings. Limit to 2 cocktails.", ingredients_str);
        println!("proompt: {}", proompt);
        let auth = Auth::from_env().unwrap();
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
        let body = CompletionsBody {
            model: "text-davinci-003".to_string(),
            max_tokens: Some(256),
            temperature: Some(0.6),
            n: Some(1),
            frequency_penalty: None,
            presence_penalty: None,
            top_p: Some(1_f32),
            stream: Some(false),
            stop: None,
            logit_bias: None,
            user: None,
            suffix: None,
            logprobs: None,
            best_of: None,
            echo: None,
            prompt: Some(vec![proompt]),
        };

        let rs = openai.completion_create(&body);
        let choice = rs.unwrap().choices;
        let message = &choice[0].text.as_ref().unwrap();
        println!("[Cocktails][ask_gpt_for_cocktails] message {:?}", message);
        CocktailData::parse_message(message)
    }
    
    // TODO: Implement pagination. Once on last page of results, start asking Chat GPT
    pub fn generate_cocktails(db: &Database, query: &GenerateQuery) -> Result<Vec<CocktailData>, Error> {
        println!("[Cocktail][generate_cocktails] ingredients: {:?}", query.ingredients);
        // 1. get all ingredients in db with similar or the same name
        let ingredients = Ingredient::get_ingredients_by_names(db, &query.ingredients)?;
        let c_ids: Vec<Uuid> = ingredients.iter().map(|x| x.cocktail_id).collect();


        let c = cocktails.filter(id.eq_any(c_ids))
            .offset(query.pagestart as i64)
            .limit(query.pagesize as i64)
            .get_results::<Cocktail>(&mut db.pool.get().unwrap())?;

        let mut cocktail_vec: Vec<CocktailData> = c.iter().map(|x| {
            let c_ings = Ingredient::get_ingredients_by_cocktail_id(db, &x.id).unwrap();
            // TODO: fix me
            // let mapped_ings = ingredients.iter().map(|x| IngredientData::from(x)).collect();
            let mapped_ings = Ingredient::map_ingredients_to_data(c_ings);
            
            let instructions = Instruction::get_instructions_by_cocktail_id(db, &x.id).unwrap();
            // TODO: fix me
            // let mapped_ins = instructions.iter().map(|x| InstructionData::from(x.clone())).collect();
            let mapped_ins = Instruction::map_instructions_to_data(instructions);

            CocktailData { 
                id: Some(x.id), 
                name: x.name.clone(),
                created_at: x.created_at,
                updated_at: x.updated_at,
                ingredients: mapped_ings, 
                instructions: mapped_ins, 
            }
        }).collect();

        if c.len() <= 1 {
            // generate cocktails from chat gippity
            let new_cocktails = Cocktail::ask_gpt_for_cocktails(&query.ingredients);
            new_cocktails.iter().for_each(|c| {
                // insert into db
                let x = Cocktail::create_cocktail(db, c.clone()).unwrap();
                let cock = Cocktail::get_cocktail_by_id(db, x.id).unwrap();
                // add to cocktails vec
                cocktail_vec.push(cock)
            });
        }

        Ok(cocktail_vec)

    }
}


#[test]
pub fn test_parse_message() {
    let test_message = "\n\n[[\"Whiskey Sour\",[[\"Whiskey\",\"2 ounces\"],[\"Lemon juice\",\"1 ounce\"],[\"Simple syrup\",\"1/2 ounce\"]],[\"Add whiskey, lemon juice and simple syrup to a shaker with ice.\",\"Shake and strain into a rocks glass with fresh ice.\",\"Garnish with a lemon wedge.\"]],\n[\"Tom Collins\",[[\"Gin\",\"2 ounces\"],[\"Lemon juice\",\"1 ounce\"],[\"Simple syrup\",\"1/2 ounce\"],[\"Club soda\",\"3 ounces\"]],[\"Fill a shaker with ice cubes.\",\"Add gin, lemon juice and simple syrup to the shaker.\",\"Shake and strain into a highball glass filled with ice.\",\"Top with club soda.\",\"Garnish with a lemon slice.\"]]]";
    let result = CocktailData::parse_message(test_message);
    let actuall: Vec<CocktailData> = vec![
        CocktailData {
            id: None,
            created_at: None,
            updated_at: None,
            name: "Whiskey Sour".to_owned(),
            ingredients: vec![
                IngredientData {
                    name: "Whiskey".to_owned(),
                    measurement: "2 ounces".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                },
                IngredientData {
                    name:"Lemon juice".to_owned(),
                    measurement: "1 ounce".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                },
                IngredientData {
                    name: "Simple syrup".to_owned(),
                    measurement: "1/2 ounce".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                },
            ],
            instructions: vec![
                InstructionData {
                    instruction: "Add whiskey, lemon juice and simple syrup to a shaker with ice.".to_owned(),
                    step:0,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },
                InstructionData {
                    instruction: "Shake and strain into a rocks glass with fresh ice.".to_owned(),
                    step:1,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },
                InstructionData {
                    instruction:"Garnish with a lemon wedge.".to_owned(),
                    step:2,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },
            ]
        },
        CocktailData {
            id: None,
            created_at: None,
            updated_at: None,
            name: "Tom Collins".to_owned(),
            ingredients: vec![
                IngredientData {
                    name: "Gin".to_owned(),
                    measurement: "2 ounces".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                },
                IngredientData {
                    name:"Lemon juice".to_owned(),
                    measurement: "1 ounce".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                },
                IngredientData {
                    name: "Simple syrup".to_owned(),
                    measurement: "1/2 ounce".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                },
                IngredientData {
                    name: "Club Soda".to_owned(),
                    measurement: "3 ounce".to_owned(),
                    id: None,
                    cocktail_id: None,
                    created_at: None, 
                    updated_at: None,
                }
            ],
            instructions: vec![
                InstructionData {
                    instruction: "Fill a shaker with ice cubes.".to_owned(),
                    step:0,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },
                InstructionData {
                    instruction: "Add gin, lemon juice and simple syrup to the shaker.".to_owned(),
                    step:1,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },
                InstructionData {
                    instruction:"Shake and strain into a highball glass filled with ice.".to_owned(),
                    step:2,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },

                InstructionData {
                    instruction:"Top with club soda.".to_owned(),
                    step:3,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                },
                InstructionData {
                    instruction:"Garnish with a lemon slice.".to_owned(),
                    step:4,
                    id: None, 
                    cocktail_id: None,
                    created_at: None,
                    updated_at: None,
                }
            ]
        }
    ];

    println!("[len tests]");
    assert_eq!(result.len(), actuall.len());

    println!("[name tests]");
    assert_eq!(result[0].name, actuall[0].name);
    assert_eq!(result[1].name, actuall[1].name);


    println!("[instructions tests]");
    assert_eq!(result[0].instructions[0].instruction, actuall[0].instructions[0].instruction);
    assert_eq!(result[1].instructions[1].instruction, actuall[1].instructions[1].instruction);

    println!("[ingredents tests]");
    assert_eq!(result[0].ingredients[0].name, actuall[0].ingredients[0].name);
    assert_eq!(result[1].ingredients[1].name, actuall[1].ingredients[1].name);
}









































