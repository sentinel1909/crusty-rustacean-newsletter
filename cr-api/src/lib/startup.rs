// src/lib/startup.rs

// configure and build an application instance

// dependencies, external and internal
use crate::authentication::reject_anonymous_users;
use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::{
    admin_dashboard, change_password, change_password_form, confirm, health_check, home, log_out,
    login, login_form, publish_newsletter, publish_newsletter_form, subscribe,
};
use crate::state::AppState;
use crate::state::ApplicationBaseUrl;
use crate::state::HmacSecret;
use crate::telemetry::MakeRequestUuid;
use anyhow::{Context, Error, Result};
use axum::{
    middleware,
    routing::{get, post},
    serve, Router,
};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_redispool::SessionRedisPool;
use redis_pool::RedisPool;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;

// struct for an Application type
pub struct Application {
    port: u16,
    listener: TcpListener,
    app: Router,
}

// implementation block to create an instance of an Application
impl Application {
    // function to build a new application instance
    pub async fn build(configuration: Settings) -> Result<Self, Error> {
        // Get database pool
        let connection_pool = get_connection_pool(&configuration.database);

        // Build a redis connection
        let redis_client = redis::Client::open(configuration.redis.uri.expose_secret().as_str())?;
        let redis_pool = RedisPool::from(redis_client);

        // Create a Redis session store
        let session_config = SessionConfig::new();
        let session_store =
            SessionStore::<SessionRedisPool>::new(Some(redis_pool.into()), session_config).await?;

        // Build an email client
        let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)
            .await
            .context("Unable to get a TCP listener...")?;
        let port = listener.local_addr()?.port();
        let app = create(
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            session_store,
        )
        .await
        .context("Failed to create the application...")?;

        Ok(Self {
            port,
            listener,
            app,
        })
    }

    // function to return the port the application is running on
    pub fn port(&self) -> u16 {
        self.port
    }

    // function to run the app until stopped
    pub async fn run_until_stopped(self) -> Result<(), Error> {
        serve(self.listener, self.app)
            .await
            .context("Unable to start the app server...")?;
        Ok(())
    }
}

// function to get a database connection pool
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

// run function
pub async fn create(
    pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>,
    session_store: SessionStore<SessionRedisPool>,
) -> Result<Router, Error> {
    // build the app state
    let app_state = AppState::create_state(
        pool,
        email_client,
        ApplicationBaseUrl(base_url),
        HmacSecret(hmac_secret),
    );

    // routes and their corresponding handlers, including setup of the Redis session, tracing, state and static assets such as css

    // routes that don't need session support
    let router_no_session = Router::new().route("/health_check", get(health_check));

    // admin section routes
    let router_for_admin_section = Router::new()
        .route("/admin/dashboard", get(admin_dashboard))
        .route("/admin/newsletter", get(publish_newsletter_form))
        .route("/admin/newsletter", post(publish_newsletter))
        .route("/admin/password", get(change_password_form))
        .route("/admin/password", post(change_password))
        .route("/admin/logout", post(log_out))
        .layer(middleware::from_fn(reject_anonymous_users));

    // All routes that need a session
    let router_for_non_admin_routes = Router::new()
        .route("/", get(home))
        .route("/login", get(login_form))
        .route("/login", post(login))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .merge(router_for_admin_section)
        .layer(SessionLayer::new(session_store));

    // master router
    let app = Router::new()
        .merge(router_no_session.merge(router_for_non_admin_routes))
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
        .with_state(app_state)
        .nest_service("/assets", ServeDir::new("assets"));

    // pass back the built server
    Ok(app)
}
