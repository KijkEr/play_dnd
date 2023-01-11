use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use play_dnd::{DBApplication, Dice};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct Qwe {
    character_name: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/character", post(get_character))
        .route("/roll_dice", get(roll_dice_api))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_character(axum::extract::Json(test): extract::Json<Qwe>) -> impl IntoResponse {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await.unwrap();

    let player_character = db.build_character(test.character_name).await;

    (StatusCode::OK, Json(player_character.character))
}

async fn roll_dice_api() -> impl IntoResponse {
    let outcome: Dice = Dice::roll_dice(10, 2);
    (StatusCode::OK, Json(outcome))
}
