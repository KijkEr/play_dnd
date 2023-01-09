use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use play_dnd::DBApplication;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/character", get(get_character))
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

async fn get_character() -> impl IntoResponse {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await.unwrap();

    let player_character = db.build_character().await;

    (StatusCode::OK, Json(player_character.character))
}

async fn roll_dice() -> impl IntoResponse {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await.unwrap();

    let player_character = db.build_character().await;

    (StatusCode::OK, Json(player_character.character))
}
