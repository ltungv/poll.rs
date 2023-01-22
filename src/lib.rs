pub mod app;
pub mod conf;
pub mod telemetry;

pub(crate) mod irv;
pub(crate) mod middleware;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod route;
pub(crate) mod service;
pub(crate) mod view;

pub(crate) const ENV_PREFIX: &str = "POLL";
pub(crate) const ENV_RUN_MODE: &str = "POLL_RUN_MODE";
pub(crate) const ENV_LOG_FILTER: &str = "POLL_LOG";

pub(crate) const CONFIG_DIRECTORY: &str = "conf";
pub(crate) const CONFIG_BASE_NAME: &str = "base";
