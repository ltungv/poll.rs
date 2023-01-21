use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde::Serialize;

use crate::{model::item::Item, service::RankingService};

use super::RouteError;

#[derive(Serialize)]
struct IndexContext {
    best_item: Option<Item>,
}

#[tracing::instrument(skip(handlebars, ranking_service))]
pub async fn get<RS>(
    handlebars: web::Data<Handlebars<'_>>,
    ranking_service: web::Data<RS>,
) -> Result<HttpResponse, RouteError>
where
    RS: RankingService,
{
    let best_item = ranking_service.get_instant_runoff_result().await?;
    let context = IndexContext { best_item };
    let body = handlebars.render("index", &context)?;
    Ok(HttpResponse::Ok().body(body))
}
