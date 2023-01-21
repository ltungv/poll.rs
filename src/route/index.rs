use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use serde::Serialize;
use tracing::log::{log, Level};
use uuid::Uuid;

use crate::{
    app::ApplicationContext,
    model::item::Item,
    service::{BallotService, RankingService},
};

use super::RouteError;

#[derive(Serialize)]
struct IndexContext {
    best_item: Option<Item>,
}

pub async fn get<IS, BS, RS>(
    identity: Option<Identity>,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
    RS: RankingService,
{
    if let Some(identity) = identity {
        {
            let uuid = match Uuid::parse_str(identity.id()?.as_str()) {
                Ok(u) => u,
                Err(err) => {
                    log!(Level::Error, "{}", err);
                    return Ok(HttpResponse::BadRequest().finish());
                }
            };
            if app_ctx.ballot_service().login(uuid).await?.is_some() {
                return Ok(HttpResponse::Found()
                    .insert_header((header::LOCATION, "/ballot"))
                    .finish());
            }
        }
    }
    let best_item = app_ctx
        .ranking_service()
        .get_instant_runoff_result()
        .await?;
    let context = IndexContext { best_item };
    let body = app_ctx.handlebars().render("index", &context)?;
    Ok(HttpResponse::Ok().body(body))
}
