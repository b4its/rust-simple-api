// src/view/GradeViewModel.rs

use serde::{Serialize, Deserialize};

// Struct untuk Input Nilai Baru (POST request)
#[derive(Debug, Deserialize)]
pub struct GradeInput {
    // Client mengirimkan ID dalam bentuk String
    pub student_id: String, 
    pub lesson_id: String,  
    pub score: f32,
    pub semester: String,
}

// Struct untuk Output Nilai ke Client
#[derive(Debug, Serialize)]
pub struct GradeOutput {
    pub id: String,
    pub student_id: String,
    pub lesson_id: String,
    pub score: f32,
    pub semester: String,
}