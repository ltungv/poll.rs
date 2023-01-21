use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{app::ApplicationContext, service::BallotService};

use super::RouteError;

#[derive(Debug, Deserialize)]
pub struct RegisterFormData {
    uuid: String,
}

#[tracing::instrument(skip(request, app_ctx))]
pub async fn post<IS, BS, RS>(
    request: HttpRequest,
    form: web::Form<RegisterFormData>,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    let uuid = app_ctx
        .ballot_service()
        .register(form.uuid.as_str())
        .await?;

    Identity::login(&request.extensions(), uuid.to_string())?;
    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
