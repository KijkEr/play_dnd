use rand::Rng;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Dice {
    pub total: i16,
}

impl Dice {
    pub fn roll_dice(die_type: i16, die_amount: i16) -> Dice {
        let mut rng = rand::thread_rng();
        let mut total: i16 = 0;

        println!("Je gooit {} keer een d{}", die_amount, die_type);

        for _n in 1..=die_amount {
            let roll = rng.gen_range(1..die_type);
            total = total + roll;
        }
        Dice { total: total }
    }
}
