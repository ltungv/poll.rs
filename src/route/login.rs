use actix_web::{cookie::Cookie, http::header, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::ApplicationContext, service::BallotService};

use super::{RouteError, IDENTITY_COOKIE_NAME};

#[derive(Deserialize)]
pub struct LoginFormData {
    uuid: Uuid,
}

pub async fn post<IS, BS, RS>(
    form: web::Form<LoginFormData>,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    match app_ctx.ballot_service().login(form.uuid).await? {
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
