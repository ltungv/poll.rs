pub mod app;
pub mod conf;
pub mod irv;
pub mod model;
pub mod repository;
pub mod route;
pub mod service;
pub mod telemetry;

pub const ENV_PREFIX: &str = "POLL";
pub const ENV_RUN_MODE: &str = "POLL_RUN_MODE";
pub const ENV_LOG_FILTER: &str = "POLL_LOG";

pub const CONFIG_DIRECTORY: &str = "conf";
pub const CONFIG_BASE_NAME: &str = "base";
