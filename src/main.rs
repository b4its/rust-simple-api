// src/main.rs

use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};

// --- Deklarasi Modul Induk ---
pub mod model;
pub mod view;
pub mod route;

// --- Import Handlers Student (CRUD) ---
use route::student_route::{
    create_student, get_all_students, get_student_by_id, update_student, delete_student
};

// --- Import Handlers Lesson (CRUD) ---
use route::lesson_route::{
    create_lesson, get_all_lessons, get_lesson_by_id, update_lesson, delete_lesson
};

// --- Import Handlers Grade (CRUD) ---
use route::grade_route::{
    create_grade, get_all_grades, get_grade_by_id, update_grade, delete_grade
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    // --- 1. INISIALISASI MONGODB ---
    let uri = "mongodb://localhost:27017"; 
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Gagal mem-parsing URI MongoDB");
    
    let client = Client::with_options(client_options)
        .expect("Gagal membuat MongoDB Client");

    println!("Server berjalan di http://127.0.0.1:8080");

    // --- 2. JALANKAN WEB SERVER ---
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            
            .service(
                web::scope("/api/v1")
                    // Route 1: STUDENT
                    .route("/students", web::post().to(create_student))       // CREATE
                    .route("/students", web::get().to(get_all_students))      // READ ALL
                    .route("/students/{id}", web::get().to(get_student_by_id))// READ BY ID
                    .route("/students/{id}", web::put().to(update_student))   // UPDATE
                    .route("/students/{id}", web::delete().to(delete_student))// DELETE
                    
                    // Route 2: LESSON
                    .route("/lessons", web::post().to(create_lesson))         // CREATE
                    .route("/lessons", web::get().to(get_all_lessons))        // READ ALL
                    .route("/lessons/{id}", web::get().to(get_lesson_by_id))  // READ BY ID
                    .route("/lessons/{id}", web::put().to(update_lesson))     // UPDATE
                    .route("/lessons/{id}", web::delete().to(delete_lesson))  // DELETE

                    // Route 3: GRADE
                    .route("/grades", web::post().to(create_grade))           // CREATE
                    .route("/grades", web::get().to(get_all_grades))          // READ ALL
                    .route("/grades/{id}", web::get().to(get_grade_by_id))    // READ BY ID
                    .route("/grades/{id}", web::put().to(update_grade))       // UPDATE
                    .route("/grades/{id}", web::delete().to(delete_grade))    // DELETE
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}