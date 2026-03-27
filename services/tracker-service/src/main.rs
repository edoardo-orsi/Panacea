use panacea_observability::ServiceConfig;

#[tokio::main]
async fn main() {
    panacea_observability::init_tracing("tracker-service")
        .expect("failed to initialise tracing for tracker-service");

    let metrics_router = panacea_observability::init_metrics();
    let config = ServiceConfig::from_env();

    let router = panacea_observability::health_router("tracker-service")
        .merge(metrics_router);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .expect("failed to bind TCP listener");

    tracing::info!(port = config.port, "service started");

    axum::serve(listener, router)
        .await
        .expect("server error");
}
