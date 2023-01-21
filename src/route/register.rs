use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};

use crate::{app::ApplicationContext, service::BallotService};

use super::RouteError;

pub async fn get<IS, BS, RS>(
    request: HttpRequest,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    let uuid = app_ctx.ballot_service().register().await?;
    Identity::login(&request.extensions(), uuid.to_string()).unwrap();
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .finish())
}
