use crate::db;
use crate::models::{Status, CreateTodo, ResultResponse};
use deadpool_postgres::{Pool, Client};
use actix_web::{web, get, post, put, Responder, HttpResponse};
use std::io::ErrorKind::Other;

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
pub async fn get_items(db_pool: web::Data<Pool>,  web::Path(id): web::Path<i32>) -> impl Responder {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to database");
    
    let result = db::get_items(&client, id).await;
    
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

#[post("/todos")]
pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodo>) -> impl Responder {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to database");
    
    println!("{}", json.title);
    let result = db::create_todo(&client, json.title.clone()).await;
    
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

#[put("/item/{list_id}/{id}")]
pub async fn check_item(db_pool: web::Data<Pool>, web::Path((list_id, id)): web::Path<(i32,  i32)>) -> impl Responder {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to database");
    
    let result = db::check_item(&client, list_id, id).await;
    
    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse{success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse{success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}