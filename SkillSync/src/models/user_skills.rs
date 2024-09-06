use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::user_skills;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserSkill {
    pub id: i32,
    pub user_id: i32,
    pub skill_id: i32,
    pub proficiency_level: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "user_skills"]
pub struct NewUserSkill {
    pub user_id: i32,
    pub skill_id: i32,
    pub proficiency_level: String,
}

#[post("/skills", format = "json", data = "<new_skill>")]
pub fn add_skill(conn: DbConn, new_skill: Json<NewSkill>) -> Result<Json<Skill>, Status> {
    use crate::schema::skills::dsl::*;

    let new_skill = NewSkill {
        name: new_skill.name.clone(),
        description: new_skill.description.clone(),
        category: new_skill.category.clone(),
    };

    diesel::insert_into(skills)
        .values(&new_skill)
        .get_result(&*conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/skills/<skill_id>")]
pub fn remove_skill(conn: DbConn, skill_id: i32) -> Result<Status, Status> {
    use crate::schema::skills::dsl::*;

    diesel::delete(skills.filter(id.eq(skill_id)))
        .execute(&*conn)
        .map(|_| Status::NoContent)
        .map_err(|_| Status::InternalServerError)
}

#[post("/user_skills", format = "json", data = "<user_skill>")]
pub fn add_user_skill(conn: DbConn, user_skill: Json<UserSkill>) -> Result<Json<UserSkill>, Status> {
    use crate::schema::user_skills;

    let new_user_skill = UserSkill {
        id: user_skill.id,
        user_id: user_skill.user_id,
        skill_id: user_skill.skill_id,
        proficiency_level: user_skill.proficiency_level.clone(),
        created_at: chrono::Local::now().naive_utc(),
    };

    diesel::insert_into(user_skills::table)
        .values(&new_user_skill)
        .get_result(&*conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/user_skills/<user_skill_id>")]
pub fn remove_user_skill(conn: DbConn, user_skill_id: i32) -> Result<Status, Status> {
    use crate::schema::user_skills::dsl::*;

    diesel::delete(user_skills.filter(id.eq(user_skill_id)))
        .execute(&*conn)
        .map(|_| Status::NoContent)
        .map_err(|_| Status::InternalServerError)
}

