use actix_web::ResponseError;

use crate::service;

pub mod ballot;
pub mod health;
pub mod index;
pub mod login;
pub mod register;

pub const IDENTITY_COOKIE_NAME: &str = "ballot-uuid";

#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error(transparent)]
    Service(#[from] service::ServiceError),
}

impl ResponseError for RouteError {}
