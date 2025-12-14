// src/route/grade_route.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use mongodb::bson::oid::ObjectId;
use crate::model::grade::Grade;
use crate::view::grade_view_model::GradeInput;

// Handler POST: Membuat Grade baru
pub async fn create_grade(
    db_client: web::Data<Client>,
    input: web::Json<GradeInput>,
) -> impl Responder {
    
    // Validasi dan konversi String ID ke ObjectId
    let student_oid = match ObjectId::parse_str(&input.student_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Student ID format"),
    };

    let lesson_oid = match ObjectId::parse_str(&input.lesson_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Lesson ID format"),
    };
    
    let db = db_client.database("school_management");
    let collection = db.collection::<Grade>("grade");
    
    let new_grade = Grade {
        id: None,
        student_id: student_oid, 
        lesson_id: lesson_oid,   
        score: input.score,
        semester: input.semester.clone(),
    };

    match collection.insert_one(new_grade).await {
        Ok(result) => {
            HttpResponse::Created().json(mongodb::bson::doc! {
                "id": result.inserted_id.to_string(),
                "message": "Grade successfully recorded"
            })
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}