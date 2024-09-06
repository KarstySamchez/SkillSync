use rocket_contrib::json::Json;
use rocket::http::Status;
use diesel::prelude::*;
use crate::db::DbConn;
use crate::models::message::{Message, NewMessage};
use crate::schema::messages;
use chrono::Utc;

#[post("/send", format = "json", data = "<new_message>")]
pub fn send_message(conn: DbConn, new_message: Json<NewMessage>) -> Result<Status, Status> {
    use crate::schema::messages::dsl::*;

    let message = NewMessage {
        sender_id: new_message.sender_id,
        receiver_id: new_message.receiver_id,
        content: new_message.content.clone(),
    };

    diesel::insert_into(messages)
        .values(&message)
        .execute(&*conn)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Status::Created)
}

#[get("/user/<user_id>/messages")]
pub fn get_messages(conn: DbConn, user_id: i32) -> Result<Json<Vec<Message>>, Status> {
    use crate::schema::messages::dsl::*;

    let results = messages
        .filter(receiver_id.eq(user_id))
        .or_filter(sender_id.eq(user_id))
        .load::<Message>(&*conn)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(results))
}

#[delete("/<message_id>")]
pub fn delete_message(conn: DbConn, message_id: i32) -> Result<Status, Status> {
    use crate::schema::messages::dsl::*;

    diesel::delete(messages.filter(id.eq(message_id)))
        .execute(&*conn)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Status::NoContent)
}
