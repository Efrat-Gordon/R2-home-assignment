use axum::{routing::post, Router};
use be_low_level::{config, handlers, models, state::AppState};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::login::login,
        handlers::logout::logout,
        handlers::try_luck::try_luck,
    ),
    components(schemas(
        models::LoginRequest,
        models::TokenResponse,
        models::LuckResponse,
    )),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("UUID")
                        .build(),
                ),
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let state = AppState {
        db,
        tokens: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/api/login", post(handlers::login::login))
        .route("/api/logout", post(handlers::logout::logout))
        .route("/api/try_luck", post(handlers::try_luck::try_luck))
        .merge(SwaggerUi::new("/api/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let addr = format!("{}:{}", config::HOST, config::PORT);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
