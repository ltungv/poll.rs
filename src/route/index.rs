use actix_web::{web, HttpResponse};
use serde::Serialize;

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

#[tracing::instrument(skip(app_ctx))]
pub async fn get<IS, BS, RS>(
    app_ctx: web::Data<ApplicationContext<'_, IS, BS, RS>>,
) -> Result<HttpResponse, RouteError>
where
    BS: BallotService,
    RS: RankingService,
{
    let best_item = app_ctx
        .ranking_service()
        .get_instant_runoff_result()
        .await?;
    let context = IndexContext { best_item };
    let body = app_ctx.handlebars().render("index", &context)?;
    Ok(HttpResponse::Ok().body(body))
}
