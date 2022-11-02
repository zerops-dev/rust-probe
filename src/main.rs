use actix_web::{get, App, HttpServer, Responder, Result};
use rustc_version_runtime;

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(String::from("Hello world!"))
}

#[get("/version")]
async fn version() -> Result<impl Responder> {
    Ok(rustc_version_runtime::version().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(version)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
