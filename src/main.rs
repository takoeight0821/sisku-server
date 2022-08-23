use std::path::Path;

use actix_cors::Cors;
use actix_web::{
    guard,
    web::{self, Data},
    App, HttpServer,
};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
// use actix_files as fs;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hovercraft(&self) -> String {
        let hovercraft_path = Path::new(
            &std::env::var("XDG_DATA_HOME")
                .unwrap_or(format!("{}/.local/share", std::env::var("HOME").unwrap())),
        )
        .join("sisku")
        .join("hovercraft");
        hovercraft_path.to_str().unwrap().to_string()
    }
}

async fn index(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(())
        .finish();
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
