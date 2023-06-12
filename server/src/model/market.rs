use mongodb::{uuid::Uuid};
use chrono::prelude::*;
enum TradeStatus {
    Pending,
    Accepted,
    Cancelled,
}

struct Trade {
    recipient_id: Uuid,
    recipient_offer: Vec<Uuid>,
    sender_id: Uuid,
    sender_offer: Vec<Uuid>,
    status: TradeStatus,
    date_finalized: DateTime<Utc>
};