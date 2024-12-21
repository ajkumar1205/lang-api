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
) -> Result<HttpResponse, actix_web::Error> {
    let mut client = client.lock().await;
    let form = form.into_inner();

    let res = client
        .post(CodeRequest::from(form))
        .await;

    match res {
        Ok(res) => {
            let response = res.into_inner();
            return Ok(HttpResponse::Ok().json(CodeResponseResult {
                output: response.body,
                time: response.time,
            }));
        }
        Err(e) => {
            let msg = e.message().to_string();
            return Err(error::ErrorBadRequest(msg));
        }
    }
}
