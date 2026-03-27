# Panacea — Project Conventions

This document is the authoritative reference for conventions that all services
and library crates in the Panacea workspace must follow.

---

## NATS Subject Naming

All subjects follow the pattern `panacea.<domain>.<event>`.

| Subject | Producer | Consumers |
|---|---|---|
| `panacea.products.scraped` | scraper-service | nutrition-service |
| `panacea.products.price_changed` | scraper-service | pricing-service, tracker-service |
| `panacea.nutrition.normalised` | nutrition-service | scoring-service, eval-service, supplement-service |
| `panacea.scoring.product_scored` | scoring-service | eval-service |
| `panacea.tracker.meal_logged` | tracker-service | supplement-service |

### Dead Letter Queue

Messages that cannot be processed after exhausting retries are republished to
a dead-letter subject:

```
panacea.dlq.<original-subject>
```

Examples:

- `panacea.dlq.products.scraped`
- `panacea.dlq.nutrition.normalised`

Consumers must publish to the DLQ rather than silently discarding failed
messages so they can be inspected, replayed, or alerted on.

---

## Error Handling

### Library crates (`panacea-core`, `panacea-observability`)

- Use `thiserror` exclusively — all errors are typed enums, never `anyhow`.
- Every error variant must have a human-readable `#[error("...")]` message.
- Do **not** use `anyhow` in library crates.

### Service crates

- Do **not** use `anyhow` — services must surface typed errors from their own
  error enums (which wrap `PanaceaError` or domain-specific variants).
- Never use `.unwrap()` or `.expect()` in production code paths — only in:
  - Test functions (`#[test]`, `#[tokio::test]`)
  - `main`, exclusively before the tracing system is initialised (i.e. the
    `init_tracing` call itself may panic if the environment is broken)
  - **Documented exception**: `ServiceConfig::from_env` panics on missing
    `DATABASE_URL` by design — a service must not start without a database
    connection string. This is documented in the struct.
- Every error returned from an HTTP handler must be logged with
  `tracing::error!` before the response is sent. Include structured fields
  (entity IDs, operation name) so the log is actionable without the trace.

---

## Logging

- Use `tracing` macros only (`tracing::info!`, `tracing::debug!`, etc.).
  Never use `println!`, `eprintln!`, or `log::` macros in production code.
- Initialise tracing via `panacea_observability::init_tracing(service_name)`
  at the very start of `main`. This sets the service name as a global
  resource attribute visible in every log line and trace span.
- Every log at `info` level or above **must** include:
  - The service name (set globally at init — no need to add it per call)
  - The relevant entity ID (e.g. `product_id`, `store_id`, `user_id`) as a
    structured field, not interpolated into the message string

### Log level guidelines

| Level | When to use |
|---|---|
| `trace` | Extremely verbose, hot-path detail. Disabled in production. |
| `debug` | Per-request or per-item detail useful during development. |
| `info` | Significant state changes: service started, job completed, event published. |
| `warn` | Recoverable anomalies: retrying a failed request, falling back to a default. |
| `error` | Failures requiring human attention: unhandled errors, data inconsistency. |

---

## Database

### Schema ownership

No service shares a schema with another. Each service owns a dedicated named
schema in TimescaleDB:

| Service | Schema |
|---|---|
| scraper-service | `scraper` |
| nutrition-service | `nutrition` |
| scoring-service | `scoring` |
| eval-service | `eval` |
| recipe-service | `recipe` |
| supplement-service | `supplement` |
| tracker-service | `tracker` |
| pricing-service | `pricing` |

### Table conventions

- All primary keys: `UUID v4` (stored as `UUID` column type in PostgreSQL)
- All timestamps: `TIMESTAMPTZ` — always UTC, never bare `TIMESTAMP`
- Every table must have:
  ```sql
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  ```
- Hypertables (TimescaleDB time-series tables) additionally require a
  `time` column which acts as the partitioning dimension

### Migrations

- Migration files live in `services/<name>/migrations/`
- Managed by `sqlx-cli` (`sqlx migrate run`)
- Migrations are numbered sequentially: `0001_initial.sql`, `0002_add_index.sql`, etc.
- Migrations are append-only — never edit a migration that has been applied to
  any environment. Write a new migration instead.

### TimescaleDB volume (pg18+)

The `timescale/timescaledb:latest-pg18` image changed its data directory
layout. The declared `VOLUME` is `/var/lib/postgresql` (the parent). All
`docker-compose.yml` volume mounts must target this parent path:

```yaml
volumes:
  - timescaledb_data:/var/lib/postgresql   # correct for pg18+
```

Mounting to `/var/lib/postgresql/data` (the old path) causes **silent data
loss** on container recreation. Do not set a `PGDATA` environment variable —
let the image default apply.
