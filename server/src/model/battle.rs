use mongodb::{uuid::Uuid};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum BattleStatus {
    Complete,
    InProgress,
    Pending,
    Error,
}
#[derive(Debug, Serialize, Deserialize)]

struct BattleInstance {
    id: Uuid,
    opponent: (PlayerBattle, PlayerBattle),
    status: BattleStatus,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerBattle {
    player_id: Uuid,
    battle_results: Option<BattleResults>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattleResults {
    damage_dealt: u32,
    damage_received: u32,
    exp_earned: u32,
    net_social_credits: i32,
}