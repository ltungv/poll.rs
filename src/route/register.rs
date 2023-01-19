use actix_web::{cookie::Cookie, http::header, web, HttpResponse};

use crate::service::BallotService;

use super::{RouteError, IDENTITY_COOKIE_NAME};

pub async fn get<BS>(ballot_service: web::Data<BS>) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    let uuid = ballot_service.register().await?;
    let cookie = Cookie::new(IDENTITY_COOKIE_NAME, uuid.to_string());
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .cookie(cookie)
        .finish())
}
