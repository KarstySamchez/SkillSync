use crate::models::{Message, NewMessage};
use diesel::prelude::*;

pub fn get_all_messages(connection: &mut PgConnection) -> Vec<Message> {
    use crate::schema::messages::dsl::*;

    let mut all_messages: Vec<Message> = Vec::new();
    let results = messages
        .select(Message::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for message in data.into_iter() {
                all_messages.push(message);
            }
            println!("Successfully retrieved messages");
        }
        Err(e) => println!("Error occurred: {:?}", e),
    }

    all_messages
}
pub fn get_messages_by_user_id(connection: &mut PgConnection, user_id_val: i32) -> Vec<Message> {
    use crate::schema::messages::dsl::*;

    let results = messages
        .filter(sender_id.eq(user_id_val).or(receiver_id.eq(user_id_val)))
        .load::<Message>(connection);

    match results {
        Ok(data) => data,
        Err(e) => {
            println!("Error occurred: {:?}", e);
            vec![]
        }}}
        pub fn add_message(new_message: NewMessage, connection: &mut PgConnection) -> Result<Message, diesel::result::Error> {
            diesel::insert_into(crate::schema::messages::table)
                .values(&new_message)
                .get_result::<Message>(connection)
        }
    
    