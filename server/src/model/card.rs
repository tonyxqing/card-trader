use mongodb::bson::uuid::Uuid;
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use image::{Rgb};
use imageproc::{definitions::{Image}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Element {
    Fire,
    Water,
    Air,
    Earth,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Skills {
    attack: Skill,
    defense: Skill,
    strength: Skill,
    hitpoints: Skill,
}

#[derive(Debug, Serialize, Deserialize,Clone)]

pub enum Rarity {
    Common, 
    Uncommon,
    Rare,
    Ascended,
    Legendary,
    Mythic,
    Cosmic,
    Galactic,
    Planetary,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum CardStyle {
    Rookie,         // Rookie rarity represents cards of up-and-coming players who are making their professional debut or early in their careers. These cards may hold special significance for collectors.
    AllStar,        // All-Star rarity represents cards of players who have been selected to participate in the All-Star Game, showcasing their exceptional skills and popularity.
    HallOfFame,     // HallOfFame rarity represents cards of players who have been inducted into the Baseball Hall of Fame, symbolizing their legendary status in the sport.
    Autograph,      // Autograph rarity represents cards that feature the authentic signature of the player. These cards are highly valued by collectors due to their personal connection with the player.
    GameUsed,       // GameUsed rarity represents cards that contain actual pieces of game-worn equipment or memorabilia, such as jerseys, bats, or baseballs. These cards are considered unique and highly sought-after.
    Parallel,       // Parallel rarity represents cards with special variants or editions, featuring different designs, colors, or finishes. These cards offer collectors additional options and variations to collect.
    Vintage,        // Vintage rarity represents cards that are considered classics or from earlier eras of baseball. These cards may have historical significance or be of interest to collectors due to their age and rarity.
    Relic,          // Relic rarity represents cards that contain embedded pieces of equipment, such as bat chips or jersey swatches. These cards are similar to GameUsed cards but focus on specific equipment components.
    Error,          // Error rarity represents cards with printing or production errors, making them unique and collectible due to their rarity and novelty.
}

impl Skills {
    pub fn new() -> Self {
        Self {
            attack: Skill::new(),
            strength: Skill::new(),
            defense: Skill::new(),
            hitpoints: Skill::new(),
        }
    }

    
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Skill {
    level: u8,
    experience: u32,
}

impl Skill {
    fn new() -> Self {
        Self {
            level: 1,
            experience: 1,
        }
    }

    fn exp_to_next_level(level: u8) -> u32 {
        let mut needed_exp = 0;
        for n in 1..level {
            needed_exp += n as u32 +  (50 * f32::powf(2.0, n as f32 / 5.0) as u32);
        }
        needed_exp
    }  

    fn level_up (&mut self) {
        if self.level != u8::MAX {
            self.level += 1;
            self.experience = 0;
        }
    }

    fn increase_experience(&mut self, exp: u32) {
        self.experience += exp;
        if self.can_level_up() {
            self.level_up();
        }
    }

    fn total_experience(&self) -> u32 {
        let mut total_exp_needed: u32 = self.experience;
        for level in 1..self.level {
            total_exp_needed += Self::exp_to_next_level(level);
        }
        total_exp_needed
    }

    fn can_level_up(&self) -> bool {
        self.experience > Self::exp_to_next_level(self.level + 1)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: Uuid,
    pub name: String,
    pub element: Element,
    pub skills: Skills,
    pub image: Vec<u8>,
    pub rarity: Rarity,
    pub card_style: CardStyle,
    pub wear: f32,
    pub owner_id: Option<Uuid>,
}

impl Card {
    pub fn new(name: String, image: Vec<u8>) -> Self {
        let mut rng = rand::thread_rng();
        let random_element = match rng.gen_range(0..=3) {
            0 => Element::Air,
            1 => Element::Earth,
            2 => Element::Fire,
            _ => Element::Water
        };

        let random_rarity = match rng.gen_range(0..=1000) {
            0..=400 => Rarity::Common,     // 40% chance
            401..=600 => Rarity::Uncommon, // 20% chance
            601..=750 => Rarity::Rare,     // 15% chance
            751..=850 => Rarity::Ascended, // 10% chance
            851..=925 => Rarity::Legendary,// 7.5% chance
            926..=975 => Rarity::Mythic,   // 5% chance
            976..=990 => Rarity::Cosmic,   // 1.5% chance
            991..=997 => Rarity::Galactic, // 0.7% chance
            _ => Rarity::Planetary,        // 0.3% chance
        };

        let random_card_style = match rng.gen_range(0..=1000) {
            0..=50 => CardStyle::Rookie,        // 5% chance
            51..=150 => CardStyle::AllStar,     // 10% chance
            151..=300 => CardStyle::HallOfFame, // 15% chance
            301..=450 => CardStyle::Autograph,  // 15% chance
            451..=600 => CardStyle::GameUsed,   // 15% chance
            601..=750 => CardStyle::Parallel,   // 15% chance
            751..=850 => CardStyle::Vintage,    // 10% chance
            851..=950 => CardStyle::Relic,      // 10% chance
            _ => CardStyle::Error,              // 5% chance
        };

        Self {
            id: Uuid::new(),
            name,
            element: random_element,
            skills: Skills::new(),
            image,
            rarity: random_rarity,
            card_style: random_card_style,
            wear: rng.gen(),
            owner_id: None,
        }
    }
    pub fn get_id (&self) -> Uuid {
        self.id
    }
    pub fn assign_owner(&mut self, owner_id: Option<Uuid>) {
        self.owner_id = owner_id;
    }
}