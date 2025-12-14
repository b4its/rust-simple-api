// src/model/Student.rs

use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    // Gunakan Option<ObjectId> untuk ID. 
    // `rename = "_id"` memetakan field Rust ke _id MongoDB.
    // `skip_serializing_if = "Option::is_none"` mencegah pengiriman field ID jika nilainya None (saat insert baru).
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub major: String,
    pub enrollment_year: u16,
}