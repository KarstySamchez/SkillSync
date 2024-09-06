use actix_web::{web, HttpResponse, Responder, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use crate::models::Endorsement;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct CreateEndorsement {
    pub user_id: i32,
    pub skill_id: i32,
    pub endorser_id: i32,
}

pub async fn get_all_endorsements(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let results = web::block(move || {
        use crate::schema::endorsements::dsl::*;
        endorsements.load::<Endorsement>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn get_endorsements_by_user(pool: web::Data<DbPool>, user_id_val: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let results = web::block(move || {
        use crate::schema::endorsements::dsl::*;
        endorsements.filter(user_id.eq(user_id_val.into_inner())).load::<Endorsement>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn create_endorsement(
    pool: web::Data<DbPool>,
    item: web::Json<CreateEndorsement>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let new_endorsement = Endorsement {
        id: 0, // Auto-incremented by the database
        user_id: item.user_id,
        skill_id: item.skill_id,
        endorser_id: item.endorser_id,
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = web::block(move || {
        use crate::schema::endorsements::dsl::*;
        diesel::insert_into(endorsements)
            .values(&new_endorsement)
            .get_result::<Endorsement>(&conn)
    })
    .await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn delete_endorsement(
    pool: web::Data<DbPool>,
    endorsement_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::endorsements::dsl::*;
        diesel::delete(endorsements.filter(id.eq(endorsement_id.into_inner()))).execute(&conn)
    })
    .await?;

    if result > 0 {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
