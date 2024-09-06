use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::skills;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "skills"]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
