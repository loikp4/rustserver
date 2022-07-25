
#[cfg(test)]
mod tests{
    use actix_web::dev::Service;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App, HttpServer};


//#[actix_web::main] // or #[tokio::main]
#[test]
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
}