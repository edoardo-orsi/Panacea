use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::types::NutritionFacts;

/// Generic event envelope wrapping any serialisable payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T: Serialize> {
    /// Unique identifier for this event instance.
    pub event_id: Uuid,
    /// Canonical event type string (mirrors the NATS subject).
    pub event_type: String,
    #[serde(with = "time::serde::rfc3339")]
    pub produced_at: OffsetDateTime,
    pub payload: T,
}

impl<T: Serialize> EventEnvelope<T> {
    /// Construct a new envelope with a freshly generated event ID and the
    /// current UTC timestamp.
    pub fn new(event_type: impl Into<String>, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event_type.into(),
            produced_at: OffsetDateTime::now_utc(),
            payload,
        }
    }
}

// ---------------------------------------------------------------------------
// Payload types
// ---------------------------------------------------------------------------

/// Emitted by scraper-service when a product page has been captured.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductScrapedPayload {
    pub product_id: Uuid,
    pub store_id: String,
    pub ean: Option<String>,
    pub name: String,
    pub brand: Option<String>,
    pub url: String,
    #[serde(with = "time::serde::rfc3339")]
    pub captured_at: OffsetDateTime,
}

impl ProductScrapedPayload {
    /// Canonical NATS subject for this event.
    pub fn subject() -> &'static str {
        "panacea.products.scraped"
    }
}

/// Emitted by scraper-service when a product price has changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChangedPayload {
    pub product_id: Uuid,
    pub store_id: String,
    pub old_price: Option<f64>,
    pub new_price: f64,
    #[serde(with = "time::serde::rfc3339")]
    pub captured_at: OffsetDateTime,
}

impl PriceChangedPayload {
    /// Canonical NATS subject for this event.
    pub fn subject() -> &'static str {
        "panacea.products.price_changed"
    }
}

/// Emitted by nutrition-service after macronutrient normalisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionNormalisedPayload {
    pub product_id: Uuid,
    pub nutrition: NutritionFacts,
    #[serde(with = "time::serde::rfc3339")]
    pub captured_at: OffsetDateTime,
}

impl NutritionNormalisedPayload {
    /// Canonical NATS subject for this event.
    pub fn subject() -> &'static str {
        "panacea.nutrition.normalised"
    }
}

/// Emitted by scoring-service after computing a product KPI score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductScoredPayload {
    pub product_id: Uuid,
    pub scorer_name: String,
    pub score: f64,
    pub scorer_version: u32,
    #[serde(with = "time::serde::rfc3339")]
    pub computed_at: OffsetDateTime,
}

impl ProductScoredPayload {
    /// Canonical NATS subject for this event.
    pub fn subject() -> &'static str {
        "panacea.scoring.product_scored"
    }
}

/// Emitted by tracker-service when the user logs a meal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealLoggedPayload {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity_g: f32,
    #[serde(with = "time::serde::rfc3339")]
    pub logged_at: OffsetDateTime,
}

impl MealLoggedPayload {
    /// Canonical NATS subject for this event.
    pub fn subject() -> &'static str {
        "panacea.tracker.meal_logged"
    }
}
