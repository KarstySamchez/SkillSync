use crate::models::{Endorsement, NewEndorsement};
use diesel::prelude::*;

pub fn get_all_endorsements(connection: &mut PgConnection) -> Vec<Endorsement> {
    use crate::schema::endorsements::dsl::*;

    let mut all_endorsements: Vec<Endorsement> = Vec::new();
    let results = endorsements
        .select(Endorsement::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for endorsement in data.into_iter() {
                all_endorsements.push(endorsement);
            }

            println!("Successfully retrieved endorsements");
        }
        Err(e) => println!("Error occurred: {:?}", e),
    }

    all_endorsements
}

pub fn get_endorsements_by_user_id(connection: &mut PgConnection, user_id_val: i32) -> Vec<Endorsement> {
    use crate::schema::endorsements::dsl::*;

    let results = endorsements
        .filter(user_id.eq(user_id_val))
        .load::<Endorsement>(connection);

    match results {
        Ok(data) => data,
        Err(e) => {
            println!("Error occurred: {:?}", e);
            vec![]
        }
    }
}

pub fn add_endorsement(new_endorsement: NewEndorsement, connection: &mut PgConnection) -> Result<Endorsement, diesel::result::Error> {
    diesel::insert_into(crate::schema::endorsements::table)
        .values(&new_endorsement)
        .get_result::<Endorsement>(connection)
}
