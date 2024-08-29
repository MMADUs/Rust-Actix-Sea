use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// Middleware 1
pub struct Middleware1 {
    value: String,
}

impl Middleware1 {
    pub fn new(value: String) -> Self {
        Middleware1 { value }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Middleware1
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Middleware1Service<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Middleware1Service {
            service,
            value: self.value.clone(),
        }))
    }
}

pub struct Middleware1Service<S> {
    service: S,
    value: String,
}

impl<S, B> Service<ServiceRequest> for Middleware1Service<S>
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
        println!("Middleware1: {}", self.value);
        req.extensions_mut().insert(self.value.clone());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
