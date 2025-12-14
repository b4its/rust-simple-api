// src/view/student_view_model.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct StudentInput {
    pub name: String,
    pub major: String,
    pub enrollment_year: u32, 
}

#[derive(Debug, Serialize)]
pub struct StudentOutput {
    pub id: String, 
    pub name: String,
    pub major: String,
    pub enrollment_year: u32, 
}