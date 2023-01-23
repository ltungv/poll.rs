use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use serde::Deserialize;

use crate::service::{BallotService, ServiceError};

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
    match ballot_service.register(form.uuid.as_str()).await {
        Ok(uuid) => {
            Identity::login(&request.extensions(), uuid.to_string())?;
            FlashMessage::new(
                "Logged in".to_string(),
                actix_web_flash_messages::Level::Success,
            )
            .send();
            Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/ballot"))
                .finish())
        }
        Err(ServiceError::Uuid(e)) => {
            tracing::warn!(error = %e, "Invalid UUID");
            FlashMessage::new(
                "Invalid UUID".to_string(),
                actix_web_flash_messages::Level::Error,
            )
            .send();
            Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish())
        }
        Err(e) => Err(e.into()),
    }
}
