use std::rc::Rc;

use crate::{database::Database, models};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    web::{block, Data},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthStatus {
    Unauthorized,
    Authorized(models::FullServerData),
}

pub struct BearerAuth;

pub struct BearerAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for BearerAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = BearerAuthMiddleware<S>;
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move {
            Ok(BearerAuthMiddleware {
                service: Rc::new(service),
            })
        })
    }
}

impl<S, B> Service<ServiceRequest> for BearerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let db = req.extract::<Data<Database>>();
        let apikey = req.headers().get("Authorization").cloned();
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            if let Some(apikey) = apikey {
                let db = db.await.unwrap();
                let server = block(move || {
                    let token = apikey
                        .to_str()
                        .unwrap()
                        .trim()
                        .strip_prefix("Bearer ")
                        .unwrap();
                    db.get_server_with_apikey(token.to_string())
                })
                .await
                .unwrap()
                .map_err(ErrorUnauthorized);

                if let Ok(serv) = server {
                    req.extensions_mut().insert(AuthStatus::Authorized(serv));
                }
            } else {
                req.extensions_mut().insert(AuthStatus::Unauthorized);
            }
            Ok(service.call(req).await.unwrap())
        })
    }
}
