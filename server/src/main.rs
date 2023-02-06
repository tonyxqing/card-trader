use std::{vec, clone, default};
use actix_web::{guard, Result, web, web::Data, App, HttpServer, Responder, HttpResponse, HttpRequest};
use async_graphql::{http::GraphiQLSource, Object, Schema, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLSubscription, GraphQLRequest, GraphQLResponse};

use std::{sync::Arc};
use futures_util::{lock::Mutex};
use slab::Slab;

struct Query;
#[Object]
impl Query {
    async fn cardholders(&self) -> std::vec::Vec<&str> {
        vec!["1", "2", "3"]
    }
}

type SchemaStruct = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Clone, Copy)]
struct Card {
    id: &'static str,
    name: &'static str,
    attack_lvl: i8,
    strength_lvl: i8,
    defence_lvl: i8,
}

impl Card {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn attack_lvl(&self) -> i8 {
        self.attack_lvl
    }

    async fn strength_lvl(&self) -> i8 {
        self.strength_lvl
    }

    async fn defence_lvl(&self) -> i8 {
        self.defence_lvl
    }
    
}

struct CardHolder {
    id: &'static str,
    name: &'static str,
    card_collection: std::vec::Vec<Card>
}

impl CardHolder {
    async fn id(&self) -> &str {
        &self.id
    }
    
    async fn name(&self) -> &str {
        &self.name
    }

    async fn card_collection(&self) -> std::vec::Vec<Card> {
        self.card_collection.to_vec()
    }
}


type Storage = Arc<Mutex<Slab<CardHolder>>>;

async fn index(schema: web::Data<SchemaStruct>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn init () -> () {

}

async fn index_ws(
    schema: web::Data<SchemaStruct>,
    req: HttpRequest,
    payload: web::Payload
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).on_connection_init(init).start(&req, payload)
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/")
            .finish()
        )
    )
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
    .data(Storage::default())
    .finish();

    HttpServer::new(move|| {
        App::new()
        .app_data(Data::new(schema.clone()))
        .service(web::resource("/").guard(guard::Post()).to(index))
        .service(
            web::resource("/")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        )
        .service(web::resource("/iql").guard(guard::Get()).to(index_graphiql))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}