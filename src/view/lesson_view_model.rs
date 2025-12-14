// src/view/lesson_view_model.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct LessonInput {
    pub name: String,
    pub credits: u32,
    pub teacher: String,
}

#[derive(Debug, Serialize)]
pub struct LessonOutput {
    pub id: String, 
    pub name: String,
    pub credits: u32,
    pub teacher: String,
}