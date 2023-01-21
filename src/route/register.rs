use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::service::BallotService;

use super::RouteError;

#[derive(Debug, Deserialize)]
pub struct RegisterFormData {
    uuid: String,
}

#[tracing::instrument(skip(request, ballot_service))]
pub async fn post<IS, BS, RS>(
    request: HttpRequest,
    form: web::Form<RegisterFormData>,
    ballot_service: web::Data<BS>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    let uuid = ballot_service.register(form.uuid.as_str()).await?;
    Identity::login(&request.extensions(), uuid.to_string())?;
    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
