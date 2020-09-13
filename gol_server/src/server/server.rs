use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    println!("GET /");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    println!("POST /echo");
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    println!("manual_hello");
    HttpResponse::Ok().body("Hey there!")
}