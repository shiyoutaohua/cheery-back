use snowflake::SnowflakeIdGenerator;
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex, OnceLock,
    },
    time::{Duration, SystemTime},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

static ID: AtomicU64 = AtomicU64::new(1);
static SNOWFLAKE: OnceLock<Mutex<SnowflakeIdGenerator>> = OnceLock::new();

pub struct SimpleIdGenerator;

impl SimpleIdGenerator {
    pub fn generate() -> u64 {
        ID.fetch_add(1, Ordering::SeqCst)
    }
}

pub struct GlobalIdGenerator;

impl GlobalIdGenerator {
    pub fn generate() -> u64 {
        let base = OffsetDateTime::parse("2022-01-01T00:00:00.000Z", &Rfc3339)
            .unwrap()
            .unix_timestamp();
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(base as u64);
        let id = SNOWFLAKE
            .get_or_init(|| Mutex::from(SnowflakeIdGenerator::with_epoch(0, 0, base)))
            .lock()
            .unwrap()
            .lazy_generate();
        id as u64
    }
}
