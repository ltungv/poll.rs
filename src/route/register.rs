use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};

use crate::{app::ApplicationContext, service::BallotService};

use super::RouteError;

pub async fn get<IS, BS, RS>(
    request: HttpRequest,
    identity: Option<Identity>,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    if identity.is_some() {
        // Redirect if there's an existing session
        return Ok(HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/ballot"))
            .finish());
    }

    let uuid = app_ctx.ballot_service().register().await?;
    Identity::login(&request.extensions(), uuid.to_string())?;

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
