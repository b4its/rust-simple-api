// src/view/StudentViewModel.rs
use serde::{Serialize, Deserialize};
// Perhatikan: Tidak perlu ObjectId di sini, karena ini adalah representasi luar.

// Struct untuk Data yang Diterima dari Client (Request Body)
#[derive(Debug, Deserialize)]
pub struct StudentInput {
    pub name: String,
    pub major: String,
    pub enrollment_year: u16,
}

// Struct untuk Data yang Dikirim ke Client (Response Body)
#[derive(Debug, Serialize)]
pub struct StudentOutput {
    // Menggunakan String untuk ID agar lebih mudah dikonsumsi oleh JSON client
    pub id: String, 
    pub name: String,
    pub major: String,
    pub enrollment_year: u16,
}