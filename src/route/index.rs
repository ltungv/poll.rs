use actix_web::{http::header, web, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use serde::Serialize;
use tracing::log::{log, Level};
use uuid::Uuid;

use crate::{
    model::item::Item,
    service::{BallotService, RankingService},
};

use super::{RouteError, IDENTITY_COOKIE_NAME};

#[derive(Serialize)]
struct IndexContext {
    best_item: Option<Item>,
}

pub async fn get<BS, RS>(
    request: HttpRequest,
    handlebars_engine: web::Data<Handlebars<'_>>,
    ballot_service: web::Data<BS>,
    ranking_service: web::Data<RS>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
    RS: RankingService,
{
    if let Some(cookie) = request.cookie(IDENTITY_COOKIE_NAME) {
        {
            let uuid = match Uuid::parse_str(cookie.value()) {
                Ok(u) => u,
                Err(err) => {
                    log!(Level::Error, "{}", err);
                    return Ok(HttpResponse::BadRequest().finish());
                }
            };
            if ballot_service.login(uuid).await?.is_some() {
                return Ok(HttpResponse::Found()
                    .insert_header((header::LOCATION, "/ballot"))
                    .finish());
            }
        }
    }
    let best_item = ranking_service.get_instant_runoff_result().await?;
    let context = IndexContext { best_item };
    let body = handlebars_engine.render("index", &context)?;
    Ok(HttpResponse::Ok().body(body))
}
