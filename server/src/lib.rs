pub mod model;
pub mod db;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{Database, bson::uuid::Uuid};
use db::{Resolver, dbplayer::*};
use model::player::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn card_can_be_traded() {}

    #[test]
    fn card_can_level_up() {}

    #[test]
    fn card_cant_attack_outside_combat_level() {}

    #[test]
    fn db_can_be_read() {
        connect_db();
    }

    #[actix_rt::test]
    async fn db_can_add_card() {
        let r: Resolver = Resolver::new().await;
        let p = Player::new(String::from("Kevin"));
        let result = add_player_to_db(&r.db, p).await;
        assert!(result);
    }

    #[actix_rt::test]
    async fn db_can_remove_card() {
        let r: Resolver = Resolver::new().await;
        remove_player_from_db(&r.db, Uuid::new());
    }

    #[actix_rt::test]
    async fn db_can_retrieve_cards() {
        let r: Resolver = Resolver::new().await;
        let players = fetch_players_from_db(&r.db).await.unwrap_or(Vec::new());
        println!("The array of players looks like {:?}", players );
    }
}
