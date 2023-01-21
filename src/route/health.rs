use actix_web::HttpResponse;

#[tracing::instrument]
pub async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}
