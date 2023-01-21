use actix_identity::Identity;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::ApplicationContext, service::BallotService};

use super::RouteError;

#[derive(Deserialize)]
pub struct LoginFormData {
    uuid: Uuid,
}

pub async fn post<IS, BS, RS>(
    request: HttpRequest,
    form: web::Form<LoginFormData>,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
{
    match app_ctx.ballot_service().login(form.uuid).await? {
        None => Ok(HttpResponse::BadRequest().finish()),
        Some(ballot) => {
            Identity::login(&request.extensions(), ballot.uuid.to_string()).unwrap();
            Ok(HttpResponse::Found()
                .insert_header((header::LOCATION, "/ballot"))
                .finish())
        }
    }
}
