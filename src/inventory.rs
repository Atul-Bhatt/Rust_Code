#[derive(Debug, PartialEq, Copy, Clone)]

pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut count_red = 0;
        let mut count_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => count_red += 1,
                ShirtColor::Blue => count_blue += 1,
            }
        }

        if count_red > count_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}

pub fn run_inventory() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);

    println!("User with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("User with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}