use std::path::Path;

use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let hovercraft_path = Path::new(
            &std::env::var("XDG_DATA_HOME")
                .unwrap_or(format!("{}/.local/share", std::env::var("HOME").unwrap())),
        )
        .join("sisku")
        .join("hovercraft");
        App::new()
            .service(fs::Files::new("/hovercraft", &hovercraft_path).show_files_listing())
            .service(fs::Files::new("/", "./sisku-react/build").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
