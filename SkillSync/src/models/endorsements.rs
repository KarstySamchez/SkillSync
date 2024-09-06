use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::endorsements;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "endorsements"]
pub struct Endorsement {
    pub id: i32,
    pub user_id: i32,
    pub skill_id: i32,
    pub endorser_id: i32,
    pub created_at: NaiveDateTime,
}
