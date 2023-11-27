use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize, FromRow)]
pub struct MemberTo {
    pub member_id: Option<u64>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}
