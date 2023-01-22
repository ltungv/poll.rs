use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use actix_web_flash_messages::{FlashMessage, Level as FlashLevel};
use serde::Deserialize;

use crate::service::BallotService;

use super::RouteError;

#[derive(Debug, Deserialize)]
pub struct LoginFormData {
    uuid: String,
}

#[tracing::instrument(skip(request, ballot_service))]
pub async fn post<IS, BS, RS>(
    request: HttpRequest,
    form: web::Form<LoginFormData>,
    ballot_service: web::Data<BS>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    match ballot_service.find_ballot(&form.uuid).await? {
        Some(ballot) => Identity::login(&request.extensions(), ballot.uuid.to_string())?,
        None => {
            FlashMessage::new("UUID not found".to_string(), FlashLevel::Error).send();
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish());
        }
    };

    FlashMessage::new("Logged in".to_string(), FlashLevel::Success).send();
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
