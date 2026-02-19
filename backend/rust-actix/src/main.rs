use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod config;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("PORT must be a valid number");

    let python_url = env::var("PYTHON_FASTAPI_URL")
        .unwrap_or_else(|_| "http://localhost:8001".into());

    log::info!("Starting Polyglot Backend on 0.0.0.0:{}", port);

    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build reqwest client");

    let state = web::Data::new(config::AppState {
        python_url,
        http_client,
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _req| {
                        actix_web::error::InternalError::from_response(
                            err,
                            actix_web::HttpResponse::BadRequest().json(
                                serde_json::json!({ "error": "Invalid JSON" }),
                            ),
                        )
                        .into()
                    }),
            )
            .app_data(state.clone())
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
