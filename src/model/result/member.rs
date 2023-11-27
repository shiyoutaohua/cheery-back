use crate::model::to::member::MemberTo;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberDetailResult {
    pub member_id: Option<u64>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberEditResult {
    pub member_id: u64,
}

impl From<MemberTo> for MemberDetailResult {
    fn from(value: MemberTo) -> Self {
        Self {
            member_id: value.member_id.clone(),
            nickname: value.nickname.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl From<&MemberTo> for MemberDetailResult {
    fn from(value: &MemberTo) -> Self {
        Self {
            member_id: value.member_id.clone(),
            nickname: value.nickname.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberLoginSuccessResult {
    pub token: String,
}
