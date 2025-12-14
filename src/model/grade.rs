// src/model/Grade.rs
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Grade {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // PENTING: Di Model, field ini harus berupa ObjectId!
    pub student_id: ObjectId, 
    pub lesson_id: ObjectId,  
    pub score: f32,
    pub semester: String,
}