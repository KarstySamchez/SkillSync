use actix_web::{web, HttpResponse, Responder, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use crate::models::Skill;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct CreateSkill {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

pub async fn get_skills(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let results = web::block(move || {
        use crate::schema::skills::dsl::*;
        skills.load::<Skill>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn get_skill(pool: web::Data<DbPool>, skill_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::skills::dsl::*;
        skills.filter(id.eq(skill_id.into_inner())).first::<Skill>(&conn)
    })
    .await;

    match result {
        Ok(skill) => HttpResponse::Ok().json(skill),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_skill(
    pool: web::Data<DbPool>,
    item: web::Json<CreateSkill>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let new_skill = Skill {
        id: 0, // This will be auto-incremented by the database
        name: item.name.clone(),
        description: item.description.clone(),
        category: item.category.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = web::block(move || {
        use crate::schema::skills::dsl::*;
        diesel::insert_into(skills)
            .values(&new_skill)
            .get_result::<Skill>(&conn)
    })
    .await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn update_skill(
    pool: web::Data<DbPool>,
    skill_id: web::Path<i32>,
    item: web::Json<CreateSkill>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::skills::dsl::*;
        diesel::update(skills.filter(id.eq(skill_id.into_inner())))
            .set((
                name.eq(&item.name),
                description.eq(&item.description),
                category.eq(&item.category),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<Skill>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_skill(
    pool: web::Data<DbPool>,
    skill_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB connection");

    let result = web::block(move || {
        use crate::schema::skills::dsl::*;
        diesel::delete(skills.filter(id.eq(skill_id.into_inner()))).execute(&conn)
    })
    .await?;

    if result > 0 {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
