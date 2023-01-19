use actix_web::HttpResponse;

pub async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}
