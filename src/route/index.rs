use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use sailfish::TemplateOnce;

use crate::{service::RankingService, view::IndexView};

use super::RouteError;

#[tracing::instrument(skip(flashes, ranking_service))]
pub async fn get<RS>(
    flashes: IncomingFlashMessages,
    ranking_service: web::Data<RS>,
) -> Result<HttpResponse, RouteError>
where
    RS: RankingService,
{
    let best_item = ranking_service.get_instant_runoff_result().await?;
    let body = IndexView::new(&best_item, &flashes).render_once()?;
    Ok(HttpResponse::Ok().body(body))
}
