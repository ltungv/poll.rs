use actix_web::{web, HttpResponse};
use sailfish::TemplateOnce;

use crate::{service::RankingService, view::index::IndexView};

use super::RouteError;

#[tracing::instrument(skip(ranking_service))]
pub async fn get<RS>(ranking_service: web::Data<RS>) -> Result<HttpResponse, RouteError>
where
    RS: RankingService,
{
    let best_item = ranking_service.get_instant_runoff_result().await?;
    let body = IndexView::new(&best_item).render_once()?;
    Ok(HttpResponse::Ok().body(body))
}
