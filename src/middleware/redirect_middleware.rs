use std::rc::Rc;

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http, HttpResponse,
};
use futures::future::{ready, LocalBoxFuture, Ready};

#[derive(Clone)]
pub struct RedirectMiddleware<F> {
    predicate: Rc<F>,
    redirected_to: &'static str,
}

impl<F> RedirectMiddleware<F> {
    pub fn new(predicate: F, redirected_to: &'static str) -> Self {
        Self {
            predicate: Rc::new(predicate),
            redirected_to,
        }
    }
}

impl<S, B, F> Transform<S, ServiceRequest> for RedirectMiddleware<F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
    F: Fn(&ServiceRequest) -> bool,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = RedirectOnHavingBallotSessionMiddlewareInner<S, F>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectOnHavingBallotSessionMiddlewareInner {
            service,
            predicate: self.predicate.clone(),
            redirected_to: self.redirected_to,
        }))
    }
}

pub struct RedirectOnHavingBallotSessionMiddlewareInner<S, F> {
    service: S,
    predicate: Rc<F>,
    redirected_to: &'static str,
}

impl<S, B, F> Service<ServiceRequest> for RedirectOnHavingBallotSessionMiddlewareInner<S, F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
    F: Fn(&ServiceRequest) -> bool,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let can_redirect = (self.predicate)(&request);
        tracing::info!(%can_redirect, path = %request.path(), "calling redirect middleware");
        if can_redirect && request.path() != self.redirected_to {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::SeeOther()
                .insert_header((http::header::LOCATION, self.redirected_to))
                .finish()
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }
        let res = self.service.call(request);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
