use std::fmt::Debug;

use actix_web::{get, web, App, HttpServer, Responder, post, HttpResponse};
use json::JsonValue;
use rustserver::route::top;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct tweetObj{
    text: String,
    date: u128,

}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/hello/new")]
async fn post_new_tweet(data: web::Form<tweetObj>) -> impl Responder
{
    dbg!(&data);

    HttpResponse::Ok().content_type("text/plain").body(format!("posted {}",data.text))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .configure(top)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
