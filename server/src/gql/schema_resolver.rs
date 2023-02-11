
use async_graphql::{Schema, Object, EmptyMutation, EmptySubscription};
pub struct Query;
#[Object]
impl Query {
    pub async fn cardholders(&self) -> Vec<&str> {
        vec!["1", "2", "3"]
    }
}

pub type SchemaStruct = Schema<Query, EmptyMutation, EmptySubscription>;


