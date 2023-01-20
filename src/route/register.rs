use actix_web::{cookie::Cookie, http::header, web, HttpResponse};

use crate::{app::ApplicationContext, service::BallotService};

use super::{RouteError, IDENTITY_COOKIE_NAME};

pub async fn get<IS, BS, RS>(
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    let uuid = app_ctx.ballot_service().register().await?;
    let cookie = Cookie::new(IDENTITY_COOKIE_NAME, uuid.to_string());
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .cookie(cookie)
        .finish())
}
