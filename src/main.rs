mod config;
mod db;
mod handlers;
mod models;


use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/user/{name}", web::get().to(greet))
            .data(pool.clone())
            .service(status)
            .service(get_todos)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
