// src/route/student_route.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use mongodb::bson::oid::ObjectId;
use crate::model::student::Student;
use crate::view::student_view_model::{StudentInput, StudentOutput};

// Handler POST: Membuat Student baru
pub async fn create_student(
    db_client: web::Data<Client>,
    input: web::Json<StudentInput>,
) -> impl Responder {
    let db = db_client.database("school_management");
    let collection = db.collection::<Student>("student");

    let new_student = Student {
        id: None,
        name: input.name.clone(),
        major: input.major.clone(),
        enrollment_year: input.enrollment_year,
    };

    match collection.insert_one(new_student).await {
        Ok(result) => {
            let output = StudentOutput {
                id: result.inserted_id.as_object_id().unwrap().to_hex(),
                name: input.name.clone(),
                major: input.major.clone(),
                enrollment_year: input.enrollment_year,
            };
            HttpResponse::Created().json(output)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}