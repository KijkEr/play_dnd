use crate::character::character::build_character;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use play_dnd::Dice;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod character;

#[derive(Deserialize)]
struct CharacterInput {
    character_name: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "play_dnd=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(root))
        .route("/character", post(get_character))
        .route("/roll_dice", get(roll_dice_api))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_character(
    State(pool): State<PgPool>,
    axum::extract::Json(character): Json<CharacterInput>,
) -> impl IntoResponse {
    let player_character = build_character(State(pool), character.character_name).await;
    println!("{:?}", player_character);
    (StatusCode::OK, Json(player_character))
}

async fn roll_dice_api() -> impl IntoResponse {
    let outcome: Dice = Dice::roll_dice(10, 2);
    (StatusCode::OK, Json(outcome))
}
