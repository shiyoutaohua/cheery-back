use crate::common::error::biz_error::BizError;
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BizResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> BizResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: BizError::OK.code,
            msg: BizError::OK.msg.to_string(),
            data: Some(data),
        }
    }
}

impl From<BizError> for BizResult<()> {
    fn from(value: BizError) -> Self {
        Self {
            code: value.code,
            msg: value.msg.to_string(),
            data: None,
        }
    }
}

impl<T> IntoResponse for BizResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
