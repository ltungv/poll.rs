use actix_web::{cookie::Cookie, http::header, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::service::BallotService;

use super::{RouteError, IDENTITY_COOKIE_NAME};

#[derive(Deserialize)]
pub struct LoginFormData {
    uuid: Uuid,
}

pub async fn post<BS>(
    form: web::Form<LoginFormData>,
    ballot_service: web::Data<BS>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    match ballot_service.login(form.uuid).await? {
        None => Ok(HttpResponse::BadRequest().finish()),
        Some(ballot) => {
            let cookie = Cookie::new(IDENTITY_COOKIE_NAME, ballot.uuid.to_string());
            Ok(HttpResponse::Found()
                .insert_header((header::LOCATION, "/ballot"))
                .cookie(cookie)
                .finish())
        }
    }
}
