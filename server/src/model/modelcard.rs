#[derive(Clone, Copy)]
pub struct Card {
    id: &'static str,
    name: &'static str,
    attack_lvl: i8,
    strength_lvl: i8,
    defence_lvl: i8,
}

impl Card {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn attack_lvl(&self) -> i8 {
        self.attack_lvl
    }

    async fn strength_lvl(&self) -> i8 {
        self.strength_lvl
    }

    async fn defence_lvl(&self) -> i8 {
        self.defence_lvl
    }
    
}

