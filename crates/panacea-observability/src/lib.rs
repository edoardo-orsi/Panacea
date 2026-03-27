use axum::{Router, routing::get};
use metrics_exporter_prometheus::PrometheusBuilder;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

// ---------------------------------------------------------------------------
// ServiceConfig
// ---------------------------------------------------------------------------

/// Configuration read from the environment at service startup.
pub struct ServiceConfig {
    /// TCP port the service will listen on. Defaults to `8080`.
    pub port: u16,
    /// Full PostgreSQL connection URL. **Required** — panics with a clear
    /// message if absent, by design (see `docs/conventions.md`).
    pub database_url: String,
    /// NATS broker URL. Defaults to `nats://nats:4222`.
    pub nats_url: String,
    /// `RUST_LOG` filter string. Defaults to `info`.
    pub rust_log: String,
    /// OTLP gRPC endpoint for Jaeger. Defaults to `http://jaeger:4317`.
    pub otel_endpoint: String,
}

impl ServiceConfig {
    /// Construct a `ServiceConfig` from environment variables.
    ///
    /// # Panics
    ///
    /// Panics if `DATABASE_URL` is not set — this is intentional. A service
    /// must not start without a database connection string.
    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!(
                "DATABASE_URL environment variable is required but was not set. \
                 Set it to a PostgreSQL connection string, e.g. \
                 postgresql://panacea:panacea@localhost:5432/panacea"
            )
        });

        let port = std::env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8080);

        Self {
            port,
            database_url,
            nats_url: std::env::var("NATS_URL")
                .unwrap_or_else(|_| "nats://nats:4222".to_string()),
            rust_log: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            otel_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://jaeger:4317".to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Tracing
// ---------------------------------------------------------------------------

/// Initialise `tracing-subscriber` with JSON output and OpenTelemetry OTLP
/// export to Jaeger.
///
/// Must be called once, early in `main`, before any tracing macros are used.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://jaeger:4317".to_string());

    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    let resource = Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        service_name.to_string(),
    )]);

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&otel_endpoint),
        )
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(resource))
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let otel_layer = tracing_opentelemetry::layer().with_tracer(
        tracer_provider.tracer(service_name.to_string()),
    );

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&rust_log));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_layer)
        .try_init()?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

/// Install the Prometheus metrics recorder and return an `axum::Router` with
/// a `GET /metrics` endpoint that renders the current Prometheus output.
pub fn init_metrics() -> Router {
    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus metrics recorder");

    Router::new().route(
        "/metrics",
        get(move || {
            let handle = handle.clone();
            async move { handle.render() }
        }),
    )
}

// ---------------------------------------------------------------------------
// Health / readiness
// ---------------------------------------------------------------------------

/// Returns an `axum::Router` with:
///
/// - `GET /health` — liveness probe, always 200
/// - `GET /ready`  — readiness probe, always 200 (wired per-service in later phases)
pub fn health_router(service_name: &'static str) -> Router {
    Router::new()
        .route(
            "/health",
            get(move || async move {
                (
                    axum::http::StatusCode::OK,
                    [(axum::http::header::CONTENT_TYPE, "application/json")],
                    format!(r#"{{"status":"ok","service":"{}"}}"#, service_name),
                )
            }),
        )
        .route(
            "/ready",
            get(move || async move {
                (
                    axum::http::StatusCode::OK,
                    [(axum::http::header::CONTENT_TYPE, "application/json")],
                    format!(r#"{{"status":"ok","service":"{}"}}"#, service_name),
                )
            }),
        )
}
