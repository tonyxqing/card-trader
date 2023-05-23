pub mod model;
pub mod db;
use futures::stream::{StreamExt, TryStreamExt};

use db::*;
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
        let db = connect_db().await;
        let p = Player::new(String::from("Kevin"));
        let result = add_player_to_db(&db.unwrap(), p).await;
        assert!(result);
    }

    #[actix_rt::test]
    async fn db_can_remove_card() {
        let db = connect_db().await;
        println!("Starting Removal");
        remove_player_from_db(&db.unwrap(), String::from("Kevin")).await;
    }
}
