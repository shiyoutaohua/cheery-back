use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    sync::{OnceLock, RwLock},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationConfiguration {
    pub app: Option<Application>,
    pub datasource: Vec<Datasource>,
    pub redis: Redis,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Datasource {
    pub name: Option<String>,
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Redis {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub static APPLICATION_CONFIGURE: OnceLock<RwLock<ApplicationConfiguration>> = OnceLock::new();
pub fn configure() {
    let _ = APPLICATION_CONFIGURE.get_or_init(|| {
        let p = env::current_dir()
            .unwrap()
            .join("res")
            .join("application.toml");
        let text = fs::read_to_string(p).expect("can't read application configuration");
        let cfg = toml::from_str(&text).unwrap();
        cfg
    });
}
