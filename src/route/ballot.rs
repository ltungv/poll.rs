use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sailfish::TemplateOnce;
use serde::Deserialize;

use crate::{
    service::{BallotService, ItemService, RankingService},
    view::BallotView,
};

use super::RouteError;

#[tracing::instrument(skip(identity, flashes, item_service, ballot_service, ranking_service))]
pub async fn get<IS, BS, RS>(
    identity: Identity,
    flashes: IncomingFlashMessages,
    item_service: web::Data<IS>,
    ballot_service: web::Data<BS>,
    ranking_service: web::Data<RS>,
) -> Result<HttpResponse, RouteError>
where
    IS: ItemService,
    BS: BallotService,
    RS: RankingService,
{
    let ballot = match ballot_service.find_ballot(&identity.id()?).await? {
        Some(v) => v,
        None => {
            Identity::logout(identity);
            FlashMessage::new(
                "Invalid session".to_string(),
                actix_web_flash_messages::Level::Error,
            )
            .send();
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish());
        }
    };
    let (best_item, (ranked_items, unranked_items)) = futures::try_join!(
        ranking_service.get_instant_runoff_result(),
        item_service.get_ballot_items(ballot.id)
    )?;
    let body = BallotView::new(
        &ballot.uuid,
        &best_item,
        &flashes,
        &ranked_items,
        &unranked_items,
    )
    .render_once()?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Debug, Deserialize)]
pub struct BallotUpdateData {
    ranked_item_ids: Vec<i32>,
}

#[tracing::instrument(skip(identity, ballot_service, ranking_service))]
pub async fn post<BS, RS>(
    identity: Identity,
    ballot_update_data: web::Json<BallotUpdateData>,
    ballot_service: web::Data<BS>,
    ranking_service: web::Data<RS>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
    RS: RankingService,
{
    let ballot = match ballot_service.find_ballot(&identity.id()?).await? {
        Some(v) => v,
        None => {
            Identity::logout(identity);
            FlashMessage::new(
                "Invalid session".to_string(),
                actix_web_flash_messages::Level::Error,
            )
            .send();
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish());
        }
    };
    ranking_service
        .update_ballot_rankings(ballot.id, &ballot_update_data.ranked_item_ids)
        .await?;
    Ok(HttpResponse::Accepted().finish())
}
