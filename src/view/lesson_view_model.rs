// src/view/LessonViewModel.rs

use serde::{Serialize, Deserialize};

// Struct untuk Input Data Mata Pelajaran Baru (POST request)
#[derive(Debug, Deserialize)]
pub struct LessonInput {
    pub name: String,
    pub credits: u8,
    pub teacher: String,
}

// Struct untuk Output Data Mata Pelajaran ke Client
#[derive(Debug, Serialize)]
pub struct LessonOutput {
    pub id: String, // ID dalam format String
    pub name: String,
    pub credits: u8,
    pub teacher: String,
}