use crate::{
    common::{error::biz_error::BizError, extractor::token::Token},
    model::dto::member::MemberSession,
};
use axum::{http::Request, middleware::Next, response::Response, Extension};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use redis::AsyncCommands;
use tracing::debug;

pub async fn member_auth<B>(
    Extension(redis_pool): Extension<Pool<RedisConnectionManager>>,
    token: Result<Token, BizError>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, BizError> {
    debug!("started processing auth");
    let whitelist = vec!["/", "/header", "/open-sse", "/member/login"];
    if whitelist.contains(&request.uri().path()) {
        debug!("whitelist matched")
    } else {
        let token = token?;
        let mut conn = redis_pool.get().await.unwrap();
        let reply: Result<String, redis::RedisError> = conn.get((*token).clone()).await;
        match reply {
            Ok(reply) => {
                let member_session: MemberSession = serde_json::from_str(&reply).unwrap();
                request.extensions_mut().insert(member_session.clone());
                debug!("find session = {:?}", member_session);
            }
            Err(error) => {
                debug!("find session error = {:?}", error);
                return Err(BizError::TOKEN_INVALID);
            }
        }
    }
    // call next service
    let response = next.run(request).await;
    // do something with `response`...
    Ok(response)
}
