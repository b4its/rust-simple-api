// src/route/grade_route.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, bson::{oid::ObjectId, doc}};
use futures::stream::TryStreamExt;
use crate::model::grade::Grade;
use crate::view::grade_view_model::{GradeInput, GradeOutput}; 

// --- Helper: Konversi Model ke Output ---
fn grade_to_output(g: Grade) -> GradeOutput {
    GradeOutput {
        id: g.id.unwrap().to_hex(), student_id: g.student_id.to_hex(), lesson_id: g.lesson_id.to_hex(), score: g.score, semester: g.semester,
    }
}

// --- CREATE (POST) ---
pub async fn create_grade(db_client: web::Data<Client>, input: web::Json<GradeInput>) -> impl Responder {
    let student_oid = match ObjectId::parse_str(&input.student_id) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Student ID format"), };
    let lesson_oid = match ObjectId::parse_str(&input.lesson_id) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Lesson ID format"), };
    
    let collection = db_client.database("school_management").collection::<Grade>("grade");
    let new_grade = Grade {
        id: None, student_id: student_oid, lesson_id: lesson_oid, score: input.score, semester: input.semester.clone(),
    };

    match collection.insert_one(new_grade).await {
        Ok(result) => HttpResponse::Created().json(doc! { 
            "id": result.inserted_id.as_object_id().unwrap().to_hex(), 
            "message": "Grade successfully recorded" 
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- READ ALL (GET /) ---
pub async fn get_all_grades(db_client: web::Data<Client>) -> impl Responder {
    let collection = db_client.database("school_management").collection::<Grade>("grade");
    match collection.find(doc! {}).await { // Hapus None
        Ok(cursor) => {
            let grades: Vec<Grade> = match cursor.try_collect().await {
                Ok(g) => g, Err(e) => return HttpResponse::InternalServerError().body(format!("Cursor error: {}", e)),
            };
            let outputs: Vec<GradeOutput> = grades.into_iter().map(grade_to_output).collect();
            HttpResponse::Ok().json(outputs)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- READ BY ID (GET /{id}) ---
pub async fn get_grade_by_id(db_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Grade ID format"), };
    let collection = db_client.database("school_management").collection::<Grade>("grade");
    match collection.find_one(doc! { "_id": object_id }).await { // Hapus None
        Ok(Some(g)) => HttpResponse::Ok().json(grade_to_output(g)),
        Ok(None) => HttpResponse::NotFound().body(format!("Grade with ID {} not found", id_str)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- UPDATE (PUT) ---
pub async fn update_grade(
    db_client: web::Data<Client>, path: web::Path<String>, new_data: web::Json<GradeInput>,
) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Grade ID format"), };
    let student_oid = match ObjectId::parse_str(&new_data.student_id) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Student ID format"), };
    let lesson_oid = match ObjectId::parse_str(&new_data.lesson_id) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Lesson ID format"), };

    let collection = db_client.database("school_management").collection::<Grade>("grade");
    let update_doc = doc! { 
        "$set": { 
            "student_id": student_oid, "lesson_id": lesson_oid, "score": new_data.score, "semester": new_data.semester.clone(),
        } 
    };
    match collection.update_one(doc! { "_id": object_id }, update_doc).await { // Hapus None
        Ok(result) => {
            if result.matched_count == 0 { return HttpResponse::NotFound().body(format!("Grade with ID {} not found", id_str)); }
            HttpResponse::Ok().json(serde_json::json!({"message": "Grade successfully updated"}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Update Error: {}", e)),
    }
}

// --- DELETE (DELETE) ---
pub async fn delete_grade(db_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Grade ID format"), };
    let collection = db_client.database("school_management").collection::<Grade>("grade");
    match collection.delete_one(doc! { "_id": object_id }).await { // Hapus None
        Ok(result) => {
            if result.deleted_count == 0 { return HttpResponse::NotFound().body(format!("Grade with ID {} not found", id_str)); }
            HttpResponse::Ok().json(serde_json::json!({"message": "Grade successfully deleted"}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Delete Error: {}", e)),
    }
}