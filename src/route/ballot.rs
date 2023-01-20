use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::log::{log, Level};
use uuid::Uuid;

use crate::{
    app::ApplicationContext,
    model::item::Item,
    service::{BallotService, ItemService, RankingService},
};

use super::{RouteError, IDENTITY_COOKIE_NAME};

#[derive(Serialize)]
struct BallotViewContext {
    uuid: Uuid,
    best_item: Option<Item>,
    ranked_items: Vec<Item>,
    unranked_items: Vec<Item>,
}

pub async fn get<IS, BS, RS>(
    request: HttpRequest,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    IS: ItemService,
    BS: BallotService,
    RS: RankingService,
{
    let cookie = match request.cookie(IDENTITY_COOKIE_NAME) {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let uuid = match Uuid::parse_str(cookie.value()) {
        Ok(u) => u,
        Err(err) => {
            log!(Level::Error, "{}", err);
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    let ballot = match app_ctx.ballot_service().login(uuid).await? {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let (best_item, (ranked_items, unranked_items)) = futures::try_join!(
        app_ctx.ranking_service().get_instant_runoff_result(),
        app_ctx.item_service().get_ballot_items(ballot.id)
    )?;

    let context = BallotViewContext {
        uuid,
        best_item,
        ranked_items,
        unranked_items,
    };
    let body = app_ctx.handlebars().render("ballot", &context)?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
pub struct BallotUpdateContext {
    ranked_item_ids: Vec<i32>,
}

pub async fn post<IS, BS, RS>(
    request: HttpRequest,
    ballot_update_data: web::Json<BallotUpdateContext>,
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
    RS: RankingService,
{
    let cookie = match request.cookie(IDENTITY_COOKIE_NAME) {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let uuid = match Uuid::parse_str(cookie.value()) {
        Ok(u) => u,
        Err(err) => {
            log!(Level::Error, "{}", err);
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    let ballot = match app_ctx.ballot_service().login(uuid).await? {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    app_ctx
        .ranking_service()
        .update_ballot_rankings(ballot.id, &ballot_update_data.ranked_item_ids)
        .await?;
    Ok(HttpResponse::Accepted().finish())
}
