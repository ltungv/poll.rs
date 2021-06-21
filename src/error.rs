use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    HandlebarsTemplate(#[from] handlebars::TemplateError),

    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl ResponseError for Error {
    // TODO: Might want to reponse differently based on the error
}
