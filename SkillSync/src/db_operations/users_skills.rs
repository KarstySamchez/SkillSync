use crate::models::UserSkill;
use diesel::prelude::*;

pub fn get_all_user_skills(connection: &mut PgConnection) -> Vec<UserSkill> {
    use crate::schema::user_skills::dsl::*;

    let mut all_user_skills: Vec<UserSkill> = Vec::new();
    let results = user_skills
        .select(UserSkill::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for user_skill in data.into_iter() {
                all_user_skills.push(user_skill);
            }
        }
        Err(e) => println!("Error occurred: {:?}", e),
    }

    all_user_skills
}

pub fn get_user_skills_by_user_id(connection: &mut PgConnection, user_id_val: i32) -> Vec<UserSkill> {
    use crate::schema::user_skills::dsl::*;

    user_skills
        .filter(user_id.eq(user_id_val))
        .load::<UserSkill>(connection)
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            vec![]
        })
}

pub fn add_user_skill(new_user_skill: UserSkill, connection: &mut PgConnection) -> Result<UserSkill, diesel::result::Error> {
    diesel::insert_into(crate::schema::user_skills::table)
        .values(&new_user_skill)
        .get_result::<UserSkill>(connection)
}
