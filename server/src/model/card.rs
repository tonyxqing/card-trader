enum Element {
    Fire,
    Water,
    Air,
    Earth,
}

struct Card {
    id: String,
    element_type: Element,
    combat_level: u8,
    owner_id: String,
    skills: Vec<String>,
}

impl Card {
    pub fn get_owner(&self) -> &str {
        &self.owner_id
    }
}