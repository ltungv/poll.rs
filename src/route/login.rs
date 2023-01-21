use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
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
            // TODO: Send flash message
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish());
        }
    };
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
