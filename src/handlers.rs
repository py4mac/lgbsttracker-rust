use crate::db;
use crate::models::Status;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, get, Responder, HttpResponse};


#[get("/status")]
pub async fn status() -> impl Responder {
    web::Json(Status { status: String::from("OK") })

}

#[get("/todos")]
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to database");
    
    let result = db::get_todos(&client).await;
    
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

#[get("/item/{id}")]
pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to database");
    
    let result = db::get_items(&client, path.0).await;
    
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}