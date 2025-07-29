use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Message {
    pub id: Uuid,
    pub value: i64,
}
