
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, error, HttpResponse};
use futures::future::{ok, Future, Either};
use tera::Tera;
use actix_http::Response;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Auth
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("{:?}", req.get_identity());

        if req.path().starts_with("/google6e03bff5229f1e21") || req.path().starts_with("/google_oauth")
            || req.path().starts_with("/assets") || req.path().eq("/login") || req.get_identity().is_some() {
            Either::A(self.service.call(req))
        } else {
            let data = req.app_data::<Tera>().unwrap();
            let templates = data.get_ref();
            let mut ctx = tera::Context::new();
            //ctx.insert("name", &name.to_owned());
            //ctx.insert("text", &"Welcome!".to_owned());
            let s = templates.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
            Either::B(ok(req.into_response(
                HttpResponse::Ok()
                    .body(s)
                    .into_body(),
            )))
        }
    }
}
