use actix_web::{get, post, web, App, HttpServer, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Deserialize)]
struct Query {
    name: String,
}

#[post("/post_hello")]
async fn hello(req: Result<web::Json<Request>, actix_web::Error>) -> impl Responder {
    match req {
        Ok(v) => {
            let name = &v.name;
            let message = format!("Hello, {}!", &name);
            info!("Post Method Received a request with name: {}", &name);
            Ok(web::Json(Response { message }))
        }
        Err(e) => {
            error!("Failed to deserialize the request: {:?}", e);
            Err(e)
        }
    }
}

#[get("/get_hello")]
async fn index(info: Result<web::Query<Query>, actix_web::Error>) -> impl Responder {
    match info {
        Ok(v) => {
            let name = &v.name;
            let message = format!("Hello, {}!", name);
            info!("Get Method Received a request with name: {}", name);
            Ok(web::Json(Response { message }))
        }
        Err(e) => {
            error!("Failed to deserialize the request: {:?}", e);
            Err(e)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Server is running at http://127.0.0.1:8000");
    HttpServer::new(|| App::new().service(hello).service(index))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
