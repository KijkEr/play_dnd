use dotenv::dotenv;
use play_dnd::DBApplication;

// #[derive(sqlx::FromRow, Debug)]

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await?;

    let character = db.get_character().await?;

    println!(
        "{} is een level {} {} {} {}",
        character.character_name,
        character.level,
        character.race,
        character.class,
        character.sub_class
    );

    Ok(())
}
