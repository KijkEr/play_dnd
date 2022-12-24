use rand::Rng;
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
    pub proficiency: i16,
}
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
pub struct Attributes {
    pub strength: f32,
    pub dexterity: f32,
    pub constitution: f32,
    pub intelligence: f32,
    pub wisdom: f32,
    pub charisma: f32,
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

pub struct Combat {
    weapon: Weapon,
    attributes: Attributes,
    character: Character,
}

impl Combat {
    pub fn new(weapon: Weapon, attributes: Attributes, character: Character) -> Combat {
        Combat {
            weapon: weapon,
            attributes: attributes,
            character: character,
        }
    }
    pub fn attack(&self) {
        let die_type = self.weapon.die_type;
        let dice_amount = self.weapon.dice_amount;
        let weapon_damage = roll_dice(die_type, dice_amount);
        let strength_mod = ((&self.attributes.strength - 10.0) / 2.0).floor();
        let strength_mod = strength_mod as i16;
        let proficiency = self.character.proficiency;

        let attack_damage = weapon_damage + strength_mod + proficiency;

        println!("Je doet {attack_damage} damage.");
    }
}

pub fn roll_dice(die_type: i16, die_amount: i16) -> i16 {
    let mut rng = rand::thread_rng();
    let mut total: i16 = 0;

    println!("Je gooit {die_amount} keer een d{die_type}");

    for _n in 1..=die_amount {
        let roll = rng.gen_range(1..die_type);
        total = total + roll;
        println!("Eerste rol is {roll}");
    }
    println!("Totale rol is {total}");
    total
}
