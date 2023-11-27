use crate::{common::cfg::app::APPLICATION_CONFIGURE, model::result::base::BizResult};
use axum::{
    extract::{Multipart, Path as PathVar, Query},
    headers::UserAgent,
    http::{header, HeaderMap},
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Extension, TypedHeader,
};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use futures::{stream, Stream};
use std::{collections::HashMap, convert::Infallible, path::Path, time::Duration};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tokio::{
    fs::OpenOptions,
    io::{AsyncWriteExt, BufWriter},
};
use tokio_stream::StreamExt;
use tokio_zookeeper::ZooKeeper;

pub async fn greet() -> BizResult<String> {
    let app_name = APPLICATION_CONFIGURE
        .get()
        .and_then(|el| el.read().ok())
        .and_then(|el| el.app.clone())
        .and_then(|el| el.name)
        .unwrap();
    let reply = format!("Hey - {}", app_name);
    BizResult::ok(reply)
}

pub async fn header(TypedHeader(user_agent): TypedHeader<UserAgent>) -> impl IntoResponse {
    BizResult::ok(format!("{:?}", user_agent))
}

pub async fn headers(header_map: HeaderMap) -> impl IntoResponse {
    BizResult::ok(format!("{:?}", header_map))
}

pub async fn path(PathVar(key): PathVar<String>) -> impl IntoResponse {
    String::from(key)
}

pub async fn query(Query(map): Query<HashMap<String, String>>) -> impl IntoResponse {
    format!("{:?}", map)
}

pub async fn post_text(body: String) -> impl IntoResponse {
    BizResult::ok(body)
}

pub async fn download_file() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment;filename=Cargo.toml".parse().unwrap(),
    );
    let data = tokio::fs::read(Path::new("Cargo.toml")).await.unwrap();
    (headers, data)
}

pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if let Some(origin_file_name) = field.file_name() {
            let dst = std::env::current_dir()
                .unwrap()
                .join("target")
                .join(origin_file_name);
            let dst = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(dst)
                .await
                .unwrap();
            let mut writer = BufWriter::new(dst);
            while let Some(data) = field.chunk().await.unwrap() {
                let _ = writer.write_all(&*data).await;
            }
            let _ = writer.flush().await;
        }
    }
    BizResult::ok(())
}

pub async fn open_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        Event::default().data(OffsetDateTime::now_utc().format(&Rfc3339).unwrap())
    })
    .map(Ok)
    .throttle(Duration::from_secs(3));
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive-msg"),
    )
}

pub async fn ping_redis(
    Extension(redis_pool): Extension<Pool<RedisConnectionManager>>,
) -> impl IntoResponse {
    let mut conn = redis_pool.get().await.unwrap();
    let reply: String = redis::cmd("PING").query_async(&mut *conn).await.unwrap();
    BizResult::ok(format!("{}", reply))
}

pub async fn ping_zk(Extension(zk): Extension<ZooKeeper>) -> impl IntoResponse {
    let state = zk.watch().exists("/zookeeper").await.unwrap();
    BizResult::ok(format!("{:?}", state))
}
