use std::path::Path;

use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder, web::Json};
// use actix_files as fs;

#[get("/")]
async fn index() -> impl Responder {
    let hovercraft_path = Path::new(
        &std::env::var("XDG_DATA_HOME")
            .unwrap_or(format!("{}/.local/share", std::env::var("HOME").unwrap())),
    )
        .join("sisku")
        .join("hovercraft");
    Json(hovercraft_path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
