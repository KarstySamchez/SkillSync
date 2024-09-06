use actix_web::{web, HttpResponse, Responder, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use crate::models::UserSkill;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct CreateUserSkill {
    pub user_id: i32,
    pub skill_id: i32,
    pub proficiency_level: String,
}

pub async fn get_user_skills(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let results = web::block(move || {
        use crate::schema::user_skills::dsl::*;
        user_skills.load::<UserSkill>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn get_user_skill(pool: web::Data<DbPool>, user_skill_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::user_skills::dsl::*;
        user_skills.filter(id.eq(user_skill_id.into_inner())).first::<UserSkill>(&conn)
    })
    .await;

    match result {
        Ok(skill) => HttpResponse::Ok().json(skill),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_user_skill(
    pool: web::Data<DbPool>,
    item: web::Json<CreateUserSkill>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let new_skill = CreateUserSkill {
        user_id: item.user_id,
        skill_id: item.skill_id,
        proficiency_level: item.proficiency_level.clone(),
    };

    let result = web::block(move || {
        use crate::schema::user_skills::dsl::*;
        diesel::insert_into(user_skills)
            .values(&new_skill)
            .get_result::<UserSkill>(&conn)
    })
    .await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn update_user_skill(
    pool: web::Data<DbPool>,
    user_skill_id: web::Path<i32>,
    item: web::Json<CreateUserSkill>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::user_skills::dsl::*;
        diesel::update(user_skills.filter(id.eq(user_skill_id.into_inner())))
            .set(&*item)
            .get_result::<UserSkill>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_user_skill(
    pool: web::Data<DbPool>,
    user_skill_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::user_skills::dsl::*;
        diesel::delete(user_skills.filter(id.eq(user_skill_id.into_inner()))).execute(&conn)
    })
    .await?;

    if result > 0 {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
