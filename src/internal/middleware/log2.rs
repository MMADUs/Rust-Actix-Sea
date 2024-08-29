use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// Middleware 2
pub struct Middleware2;

impl<S, B> Transform<S, ServiceRequest> for Middleware2
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Middleware2Service<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Middleware2Service { service }))
    }
}

pub struct Middleware2Service<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for Middleware2Service<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let value_from_middleware1 = req.extensions().get::<String>().cloned();
        if let Some(value) = value_from_middleware1 {
            println!("Middleware2: Received value from Middleware1: {}", value);
            // req.extensions_mut().insert(format!("Modified by Middleware2: {}", value));
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}