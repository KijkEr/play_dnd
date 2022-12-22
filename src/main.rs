use dotenv::dotenv;
use play_dnd::DBApplication;

// #[derive(sqlx::FromRow, Debug)]

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await?;

    let player_character = db.build_character().await;

    let weapon = player_character.weapon;
    let character = player_character.character;

    Ok(())
}
