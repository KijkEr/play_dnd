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
pub struct Attributes {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}
pub struct PlayerCharacter {
    pub character: Character,
    pub attributes: Attributes,
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
    async fn get_attributes(&self) -> Result<Attributes, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let attributes = sqlx::query_file_as!(Attributes, "./queries/attributes.sql")
            .fetch_one(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(attributes)
    }
    pub async fn build_character(&self) -> PlayerCharacter {
        let player = self.get_character().await;
        let weapon = self.get_weapon().await;
        let attributes = self.get_attributes().await;
        PlayerCharacter {
            character: player.unwrap(),
            attributes: attributes.unwrap(),
            weapon: weapon.unwrap(),
        }
    }
}
