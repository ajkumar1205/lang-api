use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/register")]
async fn register() -> impl Responder {
    HttpResponse::Ok().body("Register")
}
