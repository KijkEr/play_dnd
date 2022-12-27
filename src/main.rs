use dotenv::dotenv;
use play_dnd::DBApplication;

mod classes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await?;

    let player_character = db.build_character().await;

    let combat = classes::fighter::Combat::new(
        player_character.weapon,
        player_character.attributes,
        player_character.character,
    );

    combat.attack();

    Ok(())
}
