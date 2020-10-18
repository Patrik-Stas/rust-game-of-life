use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use gol_core::universe::universe_hash::GolStateHash;
use crate::object_cache::*;
use std::time::Duration;
use actix_web::rt::time::delay_for;

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
    std::thread::sleep(Duration::from_secs(5));
    HttpResponse::Ok().body("Hey there!")
}

#[post("/bad")]
pub async fn my_handler_bad(_req: HttpRequest) -> impl Responder {
    println!("/api/bad received");
    std::thread::sleep(Duration::from_secs(5));
    println!("/api/bad served");
    HttpResponse::Ok()
}

#[post("/good")]
pub async fn my_handler_good(_req: HttpRequest) -> impl Responder {
    println!("/api/good received");
    delay_for(Duration::from_secs(5)).await;
    println!("/api/good served");
    HttpResponse::Ok()
}

lazy_static! {
    static ref UNIVERSE_MAP: ObjectCache<GolStateHash> = ObjectCache::<GolStateHash>::new("universe-map");
}

#[post("/gols")]
pub async fn create_gol(req_body: String) -> impl Responder {
    let universe_name: String = "foo1".into();
    let mut bigbox = GolStateHash::new();
    println!("/gols request={}", req_body);
    HttpResponse::Ok().body(req_body)
}
