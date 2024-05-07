use actix_web::{web, App, HttpServer, Result};
use std::env;
use std::fs;

async fn index() -> Result<actix_web::HttpResponse> {
    let contents = fs::read_to_string("static/index.html")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read file"))?;
    Ok(actix_web::HttpResponse::Ok().body(contents))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}