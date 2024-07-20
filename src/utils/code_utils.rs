use actix_web::Responder;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct CodeRequestForm {
    pub code: String,
    pub input: String,
    pub language: Language,
}

#[derive(Deserialize)]
pub enum Language {
    Rust,
    C,
    Cpp,
}

#[derive(Serialize)]
pub struct CodeResponseResult {
    pub output: String,
    pub error: bool,
    pub edata: String,
    pub time: u64,
}
