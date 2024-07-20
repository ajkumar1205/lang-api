use crate::functions::api::CodeResponse;
use crate::functions::Client;
use crate::utils::code_utils::{CodeRequestForm, CodeResponseResult, Language};
use actix_web::{error, post, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[post("/code")]
pub async fn code(
    client: web::Data<Arc<Mutex<Client>>>,
    form: web::Json<CodeRequestForm>,
) -> impl Responder {
    let mut client = client.lock().await;

    let res: Result<CodeResponse, error::Error>;

    match form.language {
        Language::C => {
            res = client
                .c(form.code.clone(), form.input.clone())
                .await
                .map_err(|_| error::ErrorInternalServerError("Internal Server Error"));
        }
        Language::Cpp => {
            res = client
                .cpp(form.code.clone(), form.input.clone())
                .await
                .map_err(|_| error::ErrorInternalServerError("Internal Server Error"));
        }
        Language::Rust => {
            res = client
                .rust(form.code.clone(), form.input.clone())
                .await
                .map_err(|_| error::ErrorInternalServerError("Internal Server Error"));
        }
    }

    match res {
        Ok(response) => {
            let mut send = CodeResponseResult {
                output: "".to_string(),
                error: response.error,
                edata: "".to_string(),
                time: response.time,
            };

            if response.error {
                send.edata = response.body;
            } else {
                send.output = response.body;
            }
            return HttpResponse::Ok().json(send);
        }

        Err(e) => return e.into(),
    }
}
