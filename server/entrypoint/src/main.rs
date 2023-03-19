use std::net::SocketAddr;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::post,
    Router,
};
use presentation::form_handler::create_form_handler;
use resource::{database::connection::ConnectionPool, repository::Repository};
#[cfg(unix)]
use tokio::signal::unix::SignalKind;
use tower_http::cors::{Any, CorsLayer};

use crate::config::HTTP;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let conn = ConnectionPool::new().await;
    conn.migrate().await?;

    let shared_repository = Repository::new(conn).into_shared();

    let router = Router::new()
        .route("/forms", post(create_form_handler))
        .with_state(shared_repository)
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST])
                .allow_origin(Any) // todo: allow_originを制限する
                .allow_headers([CONTENT_TYPE]),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], HTTP.port.parse().unwrap()));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Fail to serve.");

    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let ctrl_c = signal(SignalKind::terminate()).unwarp();

        tokio::select! {
            _ = ctrl_c.recv() => {
                //todo: シャットダウン時にしなければいけない処理を記述する
            }
        }
    }
}
