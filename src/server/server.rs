use crate::auth::verify_token;
use crate::context::Context;
use crate::schema::Schema;
use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use env_logger;
use juniper::http::GraphQLRequest;
use log::{debug, info};
use std::sync::Arc;

pub fn init_logging() {
    env_logger::init();
}

pub async fn start_server() -> std::io::Result<()> {
    let context = Context::new();
    let schema = Arc::new(crate::schema::schema());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(context.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
    })
    .bind("127.0.0.1:8000")?
    .run();

    info!("Server running at http://localhost:8000/graphql");
    server.await
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    req: web::Json<GraphQLRequest>,
    headers: HttpRequest,
) -> HttpResponse {
    debug!("Handling GraphQL request");
    let user_id = headers
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .and_then(|token| verify_token(token));
    if user_id.is_none() {
        debug!("No valid token provided");
    } else {
        info!("Authenticated request for user: {:?}", user_id);
    }
    let res = req.execute(&st, &ctx).await;
    HttpResponse::Ok().json(res)
}
