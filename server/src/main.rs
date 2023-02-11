mod model;
mod gql;


use actix_web::{guard, web, web::Data, App, HttpServer, error, HttpResponse, HttpRequest};
use async_graphql::{http::GraphiQLSource, Schema, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLSubscription, GraphQLRequest, GraphQLResponse};


async fn index(schema: web::Data<gql::SchemaStruct>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> error::Result<HttpResponse> {
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

async fn init(value: serde_json::Value) -> async_graphql::Result<async_graphql::Data> {
	if let serde_json::Value::Object(payload) = value {
		let a = payload.get("Authorization").or(payload.get("authorization"));
		if let Some(v) = a {
			if let Some(s) = v.as_str() {
					let mut data = async_graphql::Data::default();
					data.insert(s);
					return Ok(data);
			}
		}
	}
    let err: &'static str = "hello";
	Err(err.into())
}

async fn index_ws(
    schema: web::Data<gql::SchemaStruct>,
    req: HttpRequest,
    payload: web::Payload
) -> error::Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).on_connection_init(init).start(&req, payload)
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = Schema::build(gql::Query, EmptyMutation, EmptySubscription)
    .data(model::Storage::default())
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