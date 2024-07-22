use crate::api;
use crate::api::{code_client::CodeClient, CodeRequest, CodeResponse};
use crate::utils::code_utils::{CodeRequestForm, CodeResponseResult, Language};
use actix_web::{error, post, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

#[post("/code")]
pub async fn code(
    form: web::Json<CodeRequestForm>,
    client: web::Data<Arc<Mutex<CodeClient<Channel>>>>,
) -> impl Responder {
    let mut client = client.lock().await;

    let res = client
        .post(CodeRequest {
            code: form.code.clone(),
            input: form.input.clone(),
            lang: match form.language {
                Language::C => api::Language::C as i32,
                Language::Cpp => api::Language::Cpp as i32,
                Language::Rust => api::Language::Rust as i32,
            },
        })
        .await;

    match res {
        Ok(res) => {
            let response = res.into_inner();
            return HttpResponse::Ok().json(CodeResponseResult {
                output: response.body,
                time: response.time,
            });
        }
        Err(e) => {
            return HttpResponse::from_error(error::ErrorInternalServerError(e.to_string()));
        }
    }
}
