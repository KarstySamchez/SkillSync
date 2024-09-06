use crate::models::Skill;
use diesel::prelude::*;

pub fn get_all_skills(connection: &mut PgConnection) -> Vec<Skill> {
    use crate::schema::skills::dsl::*;

    let mut all_skills: Vec<Skill> = Vec::new();
    let results = skills
        .select(Skill::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for skill in data.into_iter() {
                all_skills.push(skill);
            }
        }
        Err(e) => println!("Error occurred: {:?}", e),
    }

    all_skills
}

pub fn get_a_skill_by_name(connection: &mut PgConnection, skill_name: String) -> Option<Skill> {
    use crate::schema::skills::dsl::*;

    skills
        .filter(name.eq(skill_name))
        .first::<Skill>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn get_a_skill_by_id(connection: &mut PgConnection, skill_id: i32) -> Option<Skill> {
    use crate::schema::skills::dsl::*;

    skills
        .filter(id.eq(skill_id))
        .first::<Skill>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn add_skill(new_skill: Skill, connection: &mut PgConnection) -> Result<Skill, diesel::result::Error> {
    diesel::insert_into(crate::schema::skills::table)
        .values(&new_skill)
        .get_result::<Skill>(connection)
}
