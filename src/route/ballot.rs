use actix_web::{web, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use tokio::try_join;
use tracing::log::{log, Level};
use uuid::Uuid;

use crate::{
    model::item::Item,
    service::{BallotService, ItemService, RankingService},
};

use super::{RouteError, IDENTITY_COOKIE_NAME};

#[derive(Serialize)]
struct BallotViewContext {
    best_item: Option<Item>,
    ranked_items: Vec<Item>,
    unranked_items: Vec<Item>,
}

pub async fn get<IS, BS, RS>(
    request: HttpRequest,
    handlebars_engine: web::Data<Handlebars<'_>>,
    item_service: web::Data<IS>,
    ballot_service: web::Data<BS>,
    ranking_service: web::Data<RS>,
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

    let ballot = match ballot_service.login(uuid).await? {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let (best_item, (ranked_items, unranked_items)) = try_join!(
        ranking_service.get_instant_runoff_result(),
        item_service.get_ballot_items(ballot.id)
    )?;

    let context = BallotViewContext {
        best_item,
        ranked_items,
        unranked_items,
    };
    let body = handlebars_engine.render("ballot", &context)?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
pub struct BallotUpdateContext {
    ranked_item_ids: Vec<i32>,
}

pub async fn post<BS, RS>(
    request: HttpRequest,
    ballot_update_data: web::Json<BallotUpdateContext>,
    ballot_service: web::Data<BS>,
    ranking_service: web::Data<RS>,
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

    let ballot = match ballot_service.login(uuid).await? {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    ranking_service
        .update_ballot_rankings(ballot.id, &ballot_update_data.ranked_item_ids)
        .await?;
    Ok(HttpResponse::Accepted().finish())
}
