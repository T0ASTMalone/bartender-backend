use diesel::result::Error;
use diesel::{Queryable, RunQueryDsl, Selectable, Identifiable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::schema::instructions::dsl::*;
use crate::repository::database::Database;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Selectable, Identifiable)]
#[diesel(table_name = crate::repository::schema::instructions)]
pub struct Instruction {
    pub id: Uuid,
    pub instruction: String,
    pub step: i16,
    pub cocktail_id: Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionData {
    pub id: Option<Uuid>,
    pub instruction: String,
    pub step: i16,
    pub cocktail_id: Option<Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<(usize, String)> for InstructionData {
    fn from(tuple: (usize, String)) -> Self {
        Self {
            id: None, 
            instruction: tuple.1,
            step: tuple.0 as i16,
            cocktail_id: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl From<Instruction> for InstructionData {
    fn from(ins: Instruction) -> Self {
        Self {
           id: Some(ins.id),
           instruction: ins.instruction.clone(),
           step: ins.step,
           cocktail_id: Some(ins.cocktail_id),
           created_at: ins.created_at,
           updated_at: ins.updated_at,
        }
    }
}

impl Instruction { 
    pub fn create_instruction(db: &Database, ins: Instruction) {
        diesel::insert_into(instructions)
            .values(&ins)
            .execute(&mut db.pool.get().unwrap())
            .expect("Error inserting ingredient");
    }

    pub fn create_instructions(db: &Database, ins_vec: Vec<Instruction>) -> Result<usize, Error> {
        diesel::insert_into(instructions)
            .values(&ins_vec)
            .execute(&mut db.pool.get().unwrap())
    }

    pub fn get_instructions_by_cocktail_id(db: &Database, c_id: &Uuid) -> Result<Vec<Instruction>, Error> {
        instructions.filter(cocktail_id.eq(c_id))
            .get_results::<Instruction>(&mut db.pool.get().unwrap())
    }

    // TODO: update to use From trait
    pub fn map_instruction_to_data(ins: &Instruction) -> InstructionData {
       InstructionData {
           id: Some(ins.id),
           instruction: ins.instruction.clone(),
           step: ins.step,
           cocktail_id: Some(ins.cocktail_id),
           created_at: ins.created_at,
           updated_at: ins.updated_at,
       }
    }

    pub fn map_instructions_to_data(ins: Vec<Instruction>) -> Vec<InstructionData> {
       ins.iter().map(|x| Instruction::map_instruction_to_data(x)).collect() 
    }

    // TODO: update to use From trait
    pub fn map_data_to_instruction(ins: &InstructionData, c_id: &Uuid) -> Instruction {
       Instruction {
           id: ins.id.unwrap_or(uuid::Uuid::new_v4()),
           instruction: ins.instruction.clone(),
           step: ins.step,
           cocktail_id: ins.cocktail_id.unwrap_or(c_id.clone()),
           created_at: ins.created_at,
           updated_at: ins.updated_at,
       }
    }

    pub fn map_data_to_instructions(ins: Vec<InstructionData>, c_id: &Uuid) -> Vec<Instruction> {
       ins.iter().map(|x| Instruction::map_data_to_instruction(x, c_id)).collect() 
    }

}
