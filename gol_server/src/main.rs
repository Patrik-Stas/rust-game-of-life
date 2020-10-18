extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web, guard};
use log::Level;

use crate::server::server::{echo, hello, manual_hello, create_gol, my_handler_good, my_handler_bad};
use std::thread;
use std::sync::mpsc;
use actix_web::rt::System;

mod object_cache;
mod server;
mod error;

#[actix_web::main]
async fn main() {
    env_logger::init();

    let (tx, rx) = mpsc::channel();

    let handle_thread = thread::spawn(move || {
        let sys = System::new("http-server");

        let build_application = || {
            let scope_api = web::scope("/api")
                .guard(guard::Header("Content-Type", "application/json"))
                .service(hello)
                .service(echo)
                .service(my_handler_good)
                .service(my_handler_bad)
                .service(create_gol);

            App::new()
                .service(scope_api)
                .route("/hey", web::get().to(manual_hello))
        };
        let srv = HttpServer::new(build_application)
            .bind("127.0.0.1:7700")?
            .workers(2)
            .run();

        // srv.run();
        let _ = tx.send(srv);
        let foo = sys.run();
        foo
    });

    let srv = rx.recv().unwrap();

    println!("Pausing server");
    // pause accepting new connections
    srv.pause().await;

    println!("Resuming server");
    // resume accepting new connections
    srv.resume().await;

    handle_thread.join();
    // stop server
    // srv.stop(true).await;
}
