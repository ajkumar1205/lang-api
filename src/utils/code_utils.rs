use actix_web::Responder;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api::{self, CodeRequest};

#[derive(Deserialize)]
pub struct CodeRequestForm {
    pub code: String,
    pub input: String,
    pub language: Language,
}

impl From<CodeRequestForm> for CodeRequest {
    fn from(form: CodeRequestForm) -> Self {
        CodeRequest {
            code: form.code,
            input: form.input,
            lang: match form.language {
                Language::Cpp => api::Language::Cpp as i32,
                Language::Rust => api::Language::Rust as i32,
                Language::C => api::Language::C as i32,
            }
        }
            
    }
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
    pub time: u64,
}
