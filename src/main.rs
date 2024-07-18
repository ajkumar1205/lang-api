use actix_web::{
    get, http::StatusCode, middleware::Logger, post, web::{self, JsonBody}, App, HttpRequest, HttpResponse, HttpServer, Responder
};
use env_logger::Env;

#[get("/")]
async fn index() -> impl Responder {

    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| 
            App::new()
                .wrap(Logger::new("%a %t %r %T"))
                .service(index)
        )
        .bind("127.0.0.1:8888")?
        .run()
        .await?;
    Ok(())
}
