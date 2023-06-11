use mongodb::{uuid::Uuid};
enum BattleStatus {
    Complete,
    InProgress,
    Pending,
    Error,
}

struct BattleInstance {
    id: Uuid,
    opponent: (PlayerBattle, PlayerBattle),
}

struct PlayerBattle {
    battle_id: Uuid,
    player_id: Uuid,
    status: BattleStatus,
    battle_results: Option<BattleResults>,
}

struct BattleResults {
    damage_dealt: u32,
    damage_received: u32,
    exp_earned: u32,
    net_social_credits: i32,
}