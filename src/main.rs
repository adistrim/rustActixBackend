use actix_web::{web, App, HttpServer, Result};
use std::fs;

async fn index() -> Result<actix_web::HttpResponse> {
  let contents = fs::read_to_string("static/index.html")
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read file"))?;
  Ok(actix_web::HttpResponse::Ok().body(contents))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(index))
  })
  .run()
  .await
}
