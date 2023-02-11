use std::vec::Vec;
use crate::model::Card;

pub struct CardHolder {
    id: &'static str,
    name: &'static str,
    card_collection: Vec<Card>
}

impl CardHolder {
    async fn id(&self) -> &str {
        &self.id
    }
    
    async fn name(&self) -> &str {
        &self.name
    }

    async fn card_collection(&self) -> Vec<Card> {
        self.card_collection.to_vec()
    }
}
