mod config;
mod models;

use crate::models::Status;
use actix_web::{web, App, get, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[get("/status")]
async fn status() -> impl Responder {
    web::Json(Status { status: String::from("OK") })

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            .route("/user/{name}", web::get().to(greet))
            .service(status)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
