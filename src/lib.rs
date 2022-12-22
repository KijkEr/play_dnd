// use anyhow::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct DBApplication {
    pool: PgPool,
}
pub struct Character {
    pub character_name: String,
    pub level: i32,
    pub race: String,
    pub class: String,
    pub sub_class: String,
}
pub struct Weapon {
    pub weapon_name: String,
    pub weapon_type: String,
    pub attack_type: String,
    pub reach: String,
    pub damage: String,
    pub damage_type: String,
    pub quality: String,
}
pub struct PlayerCharacter {
    pub character: Character,
    pub weapon: Weapon,
}

impl DBApplication {
    pub async fn new(config: String) -> Result<DBApplication, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config)
            .await?;
        Ok(DBApplication { pool })
    }
    async fn get_character(&self) -> Result<Character, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let character = sqlx::query_file_as!(Character, "./queries/character.sql")
            .fetch_one(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(character)
    }
    async fn get_weapon(&self) -> Result<Weapon, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let weapon = sqlx::query_file_as!(Weapon, "./queries/weapon.sql")
            .fetch_one(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(weapon)
    }
    pub async fn build_character(&self) -> PlayerCharacter {
        let player = self.get_character().await;
        let weapon = self.get_weapon().await;
        PlayerCharacter {
            character: player.unwrap(),
            weapon: weapon.unwrap(),
        }
    }
}
