// src/view/grade_view_model.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct GradeInput {
    pub student_id: String, 
    pub lesson_id: String,  
    pub score: f32,
    pub semester: String,
}

#[derive(Debug, Serialize)]
pub struct GradeOutput {
    pub id: String,
    pub student_id: String,
    pub lesson_id: String,
    pub score: f32,
    pub semester: String,
}