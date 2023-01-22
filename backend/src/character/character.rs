use axum::extract::State;
use serde::Serialize;
use sqlx::PgPool;
#[derive(Debug, Serialize)]
pub struct Character {
    pub character_name: String,
    pub level: i32,
    pub race: String,
    pub class: String,
    pub sub_class: String,
    pub proficiency: i16,
}

#[derive(Debug)]
pub struct Attributes {
    pub strength: f32,
    pub dexterity: f32,
    pub constitution: f32,
    pub intelligence: f32,
    pub wisdom: f32,
    pub charisma: f32,
}

#[derive(Debug)]
pub struct Weapon {
    pub weapon_name: String,
    pub weapon_type: String,
    pub attack_type: String,
    pub reach: String,
    pub dice_amount: i16,
    pub die_type: i16,
    pub damage_type: String,
    pub quality: String,
}

#[derive(Debug)]
pub struct PlayerCharacter {
    pub character: Character,
    pub attributes: Attributes,
    pub weapon: Weapon,
}

async fn get_character(
    State(pool): State<PgPool>,
    character_name: String,
) -> Result<Character, sqlx::Error> {
    let character = sqlx::query_file_as!(Character, "./queries/character.sql", character_name)
        .fetch_one(&pool)
        .await?;
    Ok(character)
}

async fn get_weapon(State(pool): State<PgPool>) -> Result<Weapon, sqlx::Error> {
    let weapon = sqlx::query_file_as!(Weapon, "./queries/weapon.sql")
        .fetch_one(&pool)
        .await?;
    Ok(weapon)
}
async fn get_attributes(State(pool): State<PgPool>) -> Result<Attributes, sqlx::Error> {
    let attributes = sqlx::query_file_as!(Attributes, "./queries/attributes.sql")
        .fetch_one(&pool)
        .await?;
    Ok(attributes)
}

pub async fn build_character(
    State(pool): State<PgPool>,
    character_name: String,
) -> PlayerCharacter {
    let pool = State(pool);
    let player = get_character(pool.clone(), character_name).await;
    let weapon = get_weapon(pool.clone()).await;
    let attributes = get_attributes(pool.clone()).await;
    PlayerCharacter {
        character: player.unwrap(),
        attributes: attributes.unwrap(),
        weapon: weapon.unwrap(),
    }
}
