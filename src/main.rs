mod db;
mod functions;
mod routes;
mod utils;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use api::{code_client::CodeClient, CodeRequest, CodeResponse};
use db::DB;
use dotenv;
use env_logger::Env;
use libsql::params;
use routes::code::code;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use utils::user::User;

pub mod api {
    tonic::include_proto!("api");
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db = DB::init().await?;
    db.create_table().await?;

    let adr = format!(
        "http://{}:{}",
        dotenv::var("MIRCOSERVICE_IP")?,
        dotenv::var("MIRCOSERVICE_PORT")?
    );

    let gclient = web::Data::new(Arc::new(Mutex::new(
        CodeClient::connect(adr.clone())
            .await
            .expect("Failed to connect to the micro-service"),
    )));

    let mut state = db
        .conn
        .prepare("SELECT * FROM users WHERE verified = ?1")
        .await?;

    let mut vals = state.query(params!([1])).await?;

    let users: HashMap<String, User> = User::from(&mut vals).await;
    println!("{:?}", users);
    let data = Arc::new(Mutex::new(users));

    let app_data = web::Data::new(AppState {
        users: data.clone(),
    });

    tokio::spawn(async move {
        loop {
            let mut state = db
                .conn
                .prepare("SELECT * FROM users WHERE verified = ?1")
                .await
                .unwrap();

            let mut vals = state.query(params!([1])).await.unwrap();
            let mut d = data.lock().await;

            let users = User::from(&mut vals).await;
            println!("{:?}", users);
            *d = users;
            tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(gclient.clone())
            .wrap(Logger::new("%a %t %r %T"))
            .service(index)
            .service(code)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await?;
    Ok(())
}

struct AppState {
    pub users: Arc<Mutex<HashMap<String, User>>>,
}

