use play_dnd::{roll_dice, Attributes, Character, Weapon};

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
