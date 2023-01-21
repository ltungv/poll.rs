use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::ApplicationContext, service::BallotService};

use super::RouteError;

#[derive(Deserialize)]
pub struct LoginFormData {
    uuid: String,
}

pub async fn post<IS, BS, RS>(
    request: HttpRequest,
    identity: Option<Identity>,
    form: web::Form<LoginFormData>,
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

    let uuid = match Uuid::parse_str(&form.uuid) {
        Ok(v) => v,
        Err(_err) => {
            // TODO: Send flash message
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish());
        }
    };

    let ballot = match app_ctx.ballot_service().find_ballot(uuid).await? {
        Some(v) => v,
        None => {
            // TODO: Send flash message
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish());
        }
    };

    Identity::login(&request.extensions(), ballot.uuid.to_string())?;
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
