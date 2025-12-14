// src/route/lesson_route.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use mongodb::bson::oid::ObjectId;
use crate::model::lesson::Lesson;
use crate::view::lesson_view_model::{LessonInput, LessonOutput};

// Handler POST: Membuat Lesson baru
pub async fn create_lesson(
    db_client: web::Data<Client>,
    input: web::Json<LessonInput>,
) -> impl Responder {
    let db = db_client.database("school_management");
    let collection = db.collection::<Lesson>("lesson");

    let new_lesson = Lesson {
        id: None,
        name: input.name.clone(),
        credits: input.credits,
        teacher: input.teacher.clone(),
    };

    match collection.insert_one(new_lesson).await {
        Ok(result) => {
            let output = LessonOutput {
                id: result.inserted_id.as_object_id().unwrap().to_hex(),
                name: input.name.clone(),
                credits: input.credits,
                teacher: input.teacher.clone(),
            };
            HttpResponse::Created().json(output)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}