pub mod common;
pub mod handler;
pub mod model;
pub mod repository;
pub mod router;

use crate::common::{constant::http::HEADER_REQUEST_ID, middleware::auth::member_auth, util::app};
use axum::{extract::DefaultBodyLimit, http::HeaderName, middleware, Extension, Router};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use router::{base_router, member_router};
use sqlx::mysql::MySqlPoolOptions;
use std::{
    net::{Ipv6Addr, SocketAddr},
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use tokio_zookeeper::ZooKeeper;
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    request_id::PropagateRequestIdLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::{debug, Level};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_thread_names(true)
        .init();

    common::cfg::app::configure();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static TOKIO_WORKER_ID: AtomicUsize = AtomicUsize::new(0);
            let id = TOKIO_WORKER_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-worker-{id}")
        })
        .build()
        .expect("can't create tokio runtime");
    rt.block_on(init());
}

async fn init() {
    // database
    let db_pool = MySqlPoolOptions::new()
        .max_connections(4)
        .min_connections(1)
        .connect("mysql://qingyuehanxi:1693d1e2204c201d@mysql.sqlpub.com:3306/cheery")
        .await
        .expect("can't connect to database");
    // redis
    let redis_pool = Pool::builder()
        .max_size(4)
        .min_idle(Some(4))
        .build(RedisConnectionManager::new("redis://default:password1@119.96.133.167/").unwrap())
        .await
        .expect("can't connect to redis");
    // zookeeper
    let (zk, _watcher) = ZooKeeper::connect(&"119.96.133.167:2181".parse().unwrap())
        .await
        .expect("can't connect to zookeeper");

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(AllowMethods::mirror_request())
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(AllowCredentials::yes())
        .max_age(Duration::from_secs(3000));

    let app_router = Router::new()
        .merge(base_router::routes())
        .merge(member_router::routes())
        .fallback(app::handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(DefaultBodyLimit::max(250 * 1024 * 1024))
                .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
                    HEADER_REQUEST_ID,
                )))
                .layer(cors_layer)
                .layer(Extension(zk))
                .layer(Extension(db_pool))
                .layer(Extension(redis_pool))
                .layer(middleware::from_fn(member_auth)),
        );
    // curl http://localhost/res/application.toml
    let res_router = Router::new()
        .nest_service("/res", ServeDir::new("res"))
        .fallback(app::handler_404);

    let app_addr = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 8080);
    let res_addr = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 80);
    debug!("app listening on {}", app_addr);
    debug!("res listening on {}", res_addr);

    tokio::join!(
        app::serve(app_router, app_addr),
        app::serve(res_router, res_addr),
    );
}
