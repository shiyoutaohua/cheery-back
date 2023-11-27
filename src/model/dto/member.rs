use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberSession {
    pub member_id: Option<u64>,
    pub nickname: Option<String>,
}
