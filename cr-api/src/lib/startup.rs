//! src/lib/startup.rs

// dependencies, external and internal
use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
use axum::{
    extract::FromRef,
    http::Request,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::postgres::PgPoolOptions;
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

pub struct Application {
    port: u16,
    app: App,
}

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub db_pool: PgPool,
    pub em_client: EmailClient,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, hyper::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = match TcpListener::bind(address) {
            Ok(listener) => listener,
            Err(error) => panic!("Could not get a listener - {}", error),
        };
        let port = listener.local_addr().unwrap().port();
        let app = match run(listener, connection_pool, email_client) {
            Ok(app) => app,
            Err(error) => panic!("Could not spin up an app instance - {}", error),
        };

        Ok(Self { port, app })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> hyper::Result<()> {
        self.app.await
    }
}

// function to get a database connection pool
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

// run function
pub fn run(listener: TcpListener, pool: PgPool, email_client: EmailClient) -> hyper::Result<App> {
    // initialize the application state
    let app_state = AppState {
        db_pool: pool,
        em_client: email_client,
    };

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
        .with_state(app_state);
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
