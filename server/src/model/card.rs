use image::Rgb;
use imageproc::definitions::Image;
use mongodb::bson::uuid::Uuid;
use rand::prelude::*;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Element {
    Fire,
    Water,
    Air,
    Earth,
}
impl Element {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            0 => Element::Air,
            1 => Element::Earth,
            2 => Element::Fire,
            _ => Element::Water,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Rarity {
    Famed,
    Illustrious,
    Rare,
    Ascended,
    Legendary,
    Mythic,
    Celestial,
    Supreme,
    Immortal,
}

impl Rarity {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1000) {
              0..=400 => Rarity::Famed,         // 40% chance
            401..=600 => Rarity::Illustrious, // 20% chance
            601..=750 => Rarity::Rare,        // 15% chance
            751..=850 => Rarity::Ascended,    // 10% chance
            851..=925 => Rarity::Legendary,   // 7.5% chance
            926..=975 => Rarity::Mythic,      // 5% chance
            976..=990 => Rarity::Celestial,   // 1.5% chance
            991..=997 => Rarity::Supreme,     // 0.7% chance
                    _ => Rarity::Immortal,            // 0.3% chance
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CardStyle {
    Rookie, // Rookie rarity represents cards of up-and-coming players who are making their professional debut or early in their careers. These cards may hold special significance for collectors.
    AllStar, // All-Star rarity represents cards of players who have been selected to participate in the All-Star Game, showcasing their exceptional skills and popularity.
    HallOfFame, // HallOfFame rarity represents cards of players who have been inducted into the Baseball Hall of Fame, symbolizing their legendary status in the sport.
    Autograph, // Autograph rarity represents cards that feature the authentic signature of the player. These cards are highly valued by collectors due to their personal connection with the player.
    GameUsed, // GameUsed rarity represents cards that contain actual pieces of game-worn equipment or memorabilia, such as jerseys, bats, or baseballs. These cards are considered unique and highly sought-after.
    Parallel, // Parallel rarity represents cards with special variants or editions, featuring different designs, colors, or finishes. These cards offer collectors additional options and variations to collect.
    Vintage, // Vintage rarity represents cards that are considered classics or from earlier eras of baseball. These cards may have historical significance or be of interest to collectors due to their age and rarity.
    Relic, // Relic rarity represents cards that contain embedded pieces of equipment, such as bat chips or jersey swatches. These cards are similar to GameUsed cards but focus on specific equipment components.
    Error, // Error rarity represents cards with printing or production errors, making them unique and collectible due to their rarity and novelty.
}
impl CardStyle {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1000) {
              0..=400 => CardStyle::Rookie,
            401..=600 => CardStyle::AllStar,
            601..=750 => CardStyle::HallOfFame,
            751..=850 => CardStyle::Autograph,
            851..=925 => CardStyle::GameUsed,
            926..=975 => CardStyle::Parallel,
            976..=990 => CardStyle::Vintage,
            991..=997 => CardStyle::Relic,
                    _ => CardStyle::Error,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skills {
    attack: Skill,
    defense: Skill,
    strength: Skill,
    hitpoints: Skill,
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
            needed_exp += n as u32 + (50 * f32::powf(2.0, n as f32 / 5.0) as u32);
        }
        needed_exp
    }

    fn level_up(&mut self) {
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
    pub fn new(mut name: String, image: Vec<u8>) -> Self {
        if name.is_empty() {
            let rng = RNG::try_from(&Language::Fantasy).unwrap();
            let rng2 = RNG::try_from(&Language::Elven).unwrap();
            name = format!("{} {}", rng.generate_name(), rng2.generate_name());
        }
        Self {
            id: Uuid::new(),
            name,
            element: Element::new(),
            skills: Skills::new(),
            image,
            rarity: Rarity::new(),
            card_style: CardStyle::new(),
            wear: rand::random(),
            owner_id: None,
        }
    }
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    pub fn assign_owner(&mut self, owner_id: Option<Uuid>) {
        self.owner_id = owner_id;
    }
}
