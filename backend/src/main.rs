#[macro_use]
extern crate rocket;
use anyhow::Result;
use dotenv::dotenv;
use play_dnd::DBApplication;

mod classes;

#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await?;

    let player_character = db.build_character().await;

    let combat = classes::fighter::Combat::new(
        player_character.weapon,
        player_character.attributes,
        player_character.character,
    );
    println!("Je hebt een {}", combat.weapon.weapon_name);
    combat.attack();

    rocket::build()
        .mount("/", routes![character])
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
#[get("/")]
async fn character() -> String {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DBApplication::new(database_url).await.unwrap();

    let player_character = db.build_character().await;
    player_character.character.character_name
}
