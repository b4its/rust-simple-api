// src/route/student_route.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, bson::{oid::ObjectId, doc}};
use futures::stream::TryStreamExt;
use crate::model::student::Student;
use crate::view::student_view_model::{StudentInput, StudentOutput};

// --- CREATE (POST) ---
pub async fn create_student(
    db_client: web::Data<Client>, input: web::Json<StudentInput>,
) -> impl Responder {
    let collection = db_client.database("school_management").collection::<Student>("student");
    let new_student = Student {
        id: None, name: input.name.clone(), major: input.major.clone(), enrollment_year: input.enrollment_year,
    };
    match collection.insert_one(new_student).await {
        Ok(result) => HttpResponse::Created().json(StudentOutput {
            id: result.inserted_id.as_object_id().unwrap().to_hex(), name: input.name.clone(), major: input.major.clone(), enrollment_year: input.enrollment_year,
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- READ ALL (GET /) ---
pub async fn get_all_students(db_client: web::Data<Client>) -> impl Responder {
    let collection = db_client.database("school_management").collection::<Student>("student");
    match collection.find(doc! {}).await { // Hapus None
        Ok(cursor) => {
            let students: Vec<Student> = match cursor.try_collect().await {
                Ok(s) => s, Err(e) => return HttpResponse::InternalServerError().body(format!("Cursor error: {}", e)),
            };
            let outputs: Vec<StudentOutput> = students.into_iter().map(|s| StudentOutput {
                id: s.id.unwrap().to_hex(), name: s.name, major: s.major, enrollment_year: s.enrollment_year,
            }).collect();
            HttpResponse::Ok().json(outputs)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- READ BY ID (GET /{id}) ---
pub async fn get_student_by_id(db_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Student ID format"), };
    let collection = db_client.database("school_management").collection::<Student>("student");
    match collection.find_one(doc! { "_id": object_id }).await { // Hapus None
        Ok(Some(s)) => HttpResponse::Ok().json(StudentOutput {
            id: s.id.unwrap().to_hex(), name: s.name, major: s.major, enrollment_year: s.enrollment_year,
        }),
        Ok(None) => HttpResponse::NotFound().body(format!("Student with ID {} not found", id_str)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}

// --- UPDATE (PUT) ---
pub async fn update_student(
    db_client: web::Data<Client>, path: web::Path<String>, new_data: web::Json<StudentInput>,
) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Student ID format"), };
    let collection = db_client.database("school_management").collection::<Student>("student");
    let update_doc = doc! { "$set": { "name": new_data.name.clone(), "major": new_data.major.clone(), "enrollment_year": new_data.enrollment_year, } };
    match collection.update_one(doc! { "_id": object_id }, update_doc).await { // Hapus None
        Ok(result) => {
            if result.matched_count == 0 { return HttpResponse::NotFound().body(format!("Student with ID {} not found", id_str)); }
            HttpResponse::Ok().json(serde_json::json!({"message": "Student successfully updated"}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Update Error: {}", e)),
    }
}

// --- DELETE (DELETE) ---
pub async fn delete_student(db_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let object_id = match ObjectId::parse_str(&id_str) { Ok(oid) => oid, Err(_) => return HttpResponse::BadRequest().body("Invalid Student ID format"), };
    let collection = db_client.database("school_management").collection::<Student>("student");
    match collection.delete_one(doc! { "_id": object_id }).await { // Hapus None
        Ok(result) => {
            if result.deleted_count == 0 { return HttpResponse::NotFound().body(format!("Student with ID {} not found", id_str)); }
            HttpResponse::Ok().json(serde_json::json!({"message": "Student successfully deleted"}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Delete Error: {}", e)),
    }
}