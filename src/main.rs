use actix_cors::Cors;
use actix_web::{
    get, guard, post,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Object, Result, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
struct Hovercraft {
    entries: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
struct Entry {
    id: Uuid,
    content: String,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hovercraft<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx Hovercraft> {
        ctx.data::<Hovercraft>()
    }
}

#[post("/")]
async fn index(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[get("/")]
async fn index_playground() -> HttpResponse {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Hovercraft {
        entries: vec![Entry {
            id: Uuid::new_v4(),
            content: "Hello, world!".to_string(),
        }],
    };
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(data)
        .finish();
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .service(index)
            .service(index_playground)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
