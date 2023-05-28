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
            needed_exp += n as u32 +  f32::powf(2.0, n as f32 / 5.0) as u32;
        }
        needed_exp
    }  

    fn level_up (&mut self) {
        if self.level != u8::MAX {
            self.level += 1;
        }
    }

    fn increase_experience(&mut self, exp: u8) {
        self.experience += exp as u32;
        if self.can_level_up() {
            self.level_up();
        }
    }

    fn can_level_up(&self) -> bool {
        let mut total_exp_needed: u32 = 0;
        for level in 1..self.level - 1 {
            total_exp_needed += Self::exp_to_next_level(level);
        }
        self.experience > total_exp_needed
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: Uuid,
    pub name: String,
    pub element: Element,
    pub skills: Skills,
    pub image: Vec<u8>,
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

        Self {
            id: Uuid::new(),
            name,
            element: random_element,
            skills: Skills::new(),
            image,
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