use crate::common::error::biz_error::BizError;
use crate::model::dto::member::MemberSession;
use crate::model::param::member::{
    MemberDeleteParam, MemberDetailParam, MemberEditParam, MemberExistParam, MemberListParam,
    MemberLoginParam,
};
use crate::model::result::base::BizResult;
use crate::model::result::member::{
    MemberDetailResult, MemberEditResult, MemberLoginSuccessResult,
};
use crate::model::to::member::MemberTo;
use crate::repository::member_repository;
use axum::response::IntoResponse;
use axum::{extract::Query, Extension, Json};
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use sqlx::MySqlPool;
use std::time::Duration;
use uuid::Uuid;

pub async fn edit(
    Extension(cp): Extension<MySqlPool>,
    Json(param): Json<MemberEditParam>,
) -> impl IntoResponse {
    let mut member_to = MemberTo {
        member_id: param.member_id,
        nickname: param.nickname,
        password: param.password,
        email: param.email,
        ..Default::default()
    };
    member_repository::upsert(&cp, &mut member_to).await;
    BizResult::ok(MemberEditResult {
        member_id: member_to.member_id.unwrap(),
    })
}

pub async fn truncate(Extension(cp): Extension<MySqlPool>) -> impl IntoResponse {
    member_repository::truncate(&cp).await;
    BizResult::ok(())
}

pub async fn delete_by_id(
    Extension(cp): Extension<MySqlPool>,
    Query(param): Query<MemberDeleteParam>,
) -> impl IntoResponse {
    let ok = member_repository::delete_by_id(&cp, param.member_id).await;
    BizResult::ok(ok)
}

pub async fn count(Extension(cp): Extension<MySqlPool>) -> impl IntoResponse {
    let count = member_repository::count(&cp).await;
    BizResult::ok(count)
}

pub async fn all(Extension(cp): Extension<MySqlPool>) -> impl IntoResponse {
    let vec: Vec<MemberDetailResult> = member_repository::find_all(&cp)
        .await
        .iter()
        .map(|el| MemberDetailResult::from(el))
        .collect();
    BizResult::ok(vec)
}

pub async fn exist_by_id(
    Extension(cp): Extension<MySqlPool>,
    Query(param): Query<MemberExistParam>,
) -> impl IntoResponse {
    let exist = member_repository::exist_by_id(&cp, param.member_id).await;
    BizResult::ok(exist)
}

pub async fn detail_by_id(
    Extension(cp): Extension<MySqlPool>,
    Query(param): Query<MemberDetailParam>,
) -> impl IntoResponse {
    let member_to = member_repository::find_by_id(&cp, param.member_id).await;
    let r = member_to.and_then(|el| Some(MemberDetailResult::from(el)));
    BizResult::ok(r)
}

pub async fn list_by_id(
    Extension(cp): Extension<MySqlPool>,
    Json(param): Json<MemberListParam>,
) -> impl IntoResponse {
    let payload: Vec<MemberDetailResult> = match param.member_ids {
        Some(member_ids) => member_repository::list_by_id(&cp, &member_ids)
            .await
            .iter()
            .map(|el| MemberDetailResult::from(el))
            .collect(),
        None => Default::default(),
    };
    BizResult::ok(payload)
}

pub async fn login(
    Extension(cp): Extension<MySqlPool>,
    Extension(redis_pool): Extension<Pool<RedisConnectionManager>>,
    Json(param): Json<MemberLoginParam>,
) -> Result<impl IntoResponse, BizError> {
    let member_to = member_repository::find_by_email(&cp, &param.email).await;
    if let Some(member_to) = member_to {
        if Some(param.password) == member_to.password {
            let rand_factor = Uuid::new_v4().as_simple().to_string();
            let token = format!("token_{}_{}", member_to.member_id.unwrap(), rand_factor);
            let member_session = MemberSession {
                member_id: member_to.member_id,
                nickname: member_to.nickname,
            };
            let member_session = serde_json::to_string(&member_session).unwrap_or("".into());
            let mut conn = redis_pool.get().await.unwrap();
            let _: () = conn
                .set_ex(
                    token.clone(),
                    member_session,
                    Duration::from_secs(1 * 60 * 60)
                        .as_secs()
                        .try_into()
                        .unwrap(),
                )
                .await
                .unwrap();
            return Ok(BizResult::ok(MemberLoginSuccessResult {
                token: token.to_string(),
            }));
        }
    }
    Err(BizError::EMAIL_PASSWORD_INCORRECT)
}
