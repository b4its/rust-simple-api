// src/route/lesson_route.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, bson::{oid::ObjectId, doc}};
use futures::stream::TryStreamExt;
use crate::model::lesson::Lesson;
use crate::view::lesson_view_model::{LessonInput, LessonOutput}; 

// --- CREATE (POST) ---
pub async fn create_lesson(db_client: web::Data<Client>, input: web::Json<LessonInput>) -> impl Responder {
    let collection = db_client.database("school_management").collection::<Lesson>("lesson");
    let new_lesson = Lesson {
        id: None, name: input.name.clone(), credits: input.credits, teacher: input.teacher.clone(),
    };
    match collection.insert_one(new_lesson).await {
        Ok(result) => HttpResponse::Created().json(LessonOutput {
            id: result.inserted_id.as_object_id().unwrap().to_hex(), name: input.name.clone(), credits: input.credits, teacher: input.teacher.clone(),
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- READ ALL (GET /) ---
pub async fn get_all_lessons(db_client: web::Data<Client>) -> impl Responder {
    let collection = db_client.database("school_management").collection::<Lesson>("lesson");
    match collection.find(doc! {}).await { // Hapus None
        Ok(cursor) => {
            let lessons: Vec<Lesson> = match cursor.try_collect().await {
                Ok(l) => l, Err(e) => return HttpResponse::InternalServerError().body(format!("Cursor error: {}", e)),
            };
            let outputs: Vec<LessonOutput> = lessons.into_iter().map(|l| LessonOutput {
                id: l.id.unwrap().to_hex(), name: l.name, credits: l.credits, teacher: l.teacher,
            }).collect();
            HttpResponse::Ok().json(outputs)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- READ BY ID (GET /{id}) ---
pub async fn get_lesson_by_id(db_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Lesson ID format"), };
    let collection = db_client.database("school_management").collection::<Lesson>("lesson");
    match collection.find_one(doc! { "_id": object_id }).await { // Hapus None
        Ok(Some(l)) => HttpResponse::Ok().json(LessonOutput {
            id: l.id.unwrap().to_hex(), name: l.name, credits: l.credits, teacher: l.teacher,
        }),
        Ok(None) => HttpResponse::NotFound().body(format!("Lesson with ID {} not found", id_str)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- UPDATE (PUT) ---
pub async fn update_lesson(
    db_client: web::Data<Client>, path: web::Path<String>, new_data: web::Json<LessonInput>,
) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Lesson ID format"), };
    let collection = db_client.database("school_management").collection::<Lesson>("lesson");
    let update_doc = doc! { "$set": { "name": new_data.name.clone(), "credits": new_data.credits, "teacher": new_data.teacher.clone(), } };
    match collection.update_one(doc! { "_id": object_id }, update_doc).await { // Hapus None
        Ok(result) => {
            if result.matched_count == 0 { return HttpResponse::NotFound().body(format!("Lesson with ID {} not found", id_str)); }
            HttpResponse::Ok().json(serde_json::json!({"message": "Lesson successfully updated"}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Update Error: {}", e)),
    }
}

// --- DELETE (DELETE) ---
pub async fn delete_lesson(db_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Lesson ID format"), };
    let collection = db_client.database("school_management").collection::<Lesson>("lesson");
    match collection.delete_one(doc! { "_id": object_id }).await { // Hapus None
        Ok(result) => {
            if result.deleted_count == 0 { return HttpResponse::NotFound().body(format!("Lesson with ID {} not found", id_str)); }
            HttpResponse::Ok().json(serde_json::json!({"message": "Lesson successfully deleted"}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Delete Error: {}", e)),
    }
}