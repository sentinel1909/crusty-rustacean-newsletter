//! src/lib/startup.rs

// dependcencies, external and internal
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
use axum::{
    http::Request,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, RequestId},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
use uuid::Uuid;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

// run function
pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    // routes and their corresponding handlers
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
        .with_state(pool);
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
