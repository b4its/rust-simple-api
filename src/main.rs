// src/main.rs
use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};

// --- Deklarasi Modul Induk (Direktori) ---
// Ini mengacu pada folder src/model/, src/view/, dan src/route/
pub mod model;
pub mod view;
pub mod route;

// --- Import Handlers dari Sub-Modul Route ---
// PENTING: Jika nama file route Anda adalah 'student_route.rs', 
// maka Anda harus mengimpornya sebagai 'route::student_route'
use route::student_route::create_student;
use route::lesson_route::create_lesson;
use route::grade_route::create_grade;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    // --- 1. INISIALISASI MONGODB ---
    let uri = "mongodb://localhost:27017"; 
    
    // KlienOptions::parse adalah async
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Gagal mem-parsing URI MongoDB");
    
    let client = Client::with_options(client_options)
        .expect("Gagal membuat MongoDB Client");

    println!("Server berjalan di http://127.0.0.1:8080");

    // --- 2. JALANKAN WEB SERVER ---
    HttpServer::new(move || {
        App::new()
            // Bagikan Klien MongoDB ke semua handler
            // Gunakan .clone() karena setiap thread worker memerlukan salinan Client
            .app_data(web::Data::new(client.clone()))
            
            // Konfigurasi Route
            .service(
                web::scope("/api/v1")
                    // Rute Student
                    .route("/students", web::post().to(create_student)) 
                    // Rute Lesson
                    .route("/lessons", web::post().to(create_lesson)) 
                    // Rute Grade
                    .route("/grades", web::post().to(create_grade)) 
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}