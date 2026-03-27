use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Opaque identifier for a product in the Panacea system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductId(pub Uuid);

/// Identifies a retail store or data source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StoreId(pub String);

/// European Article Number barcode.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EanCode(pub String);

/// Macronutrient profile for a product, all values per 100 g.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionFacts {
    pub kcal: f32,
    pub protein_g: f32,
    pub fat_g: f32,
    pub saturated_fat_g: f32,
    pub carbs_g: f32,
    pub sugars_g: f32,
    pub fibre_g: f32,
    pub salt_g: f32,
}

/// A single micronutrient measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicronutrientFact {
    /// Canonical nutrient identifier (e.g. `"vitamin_c"`).
    pub nutrient_id: String,
    pub value_mg: f32,
    pub per_100g: bool,
}

/// A price observation for a product at a specific store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub product_id: ProductId,
    pub store_id: StoreId,
    pub price: f64,
    pub unit_price: Option<f64>,
    pub currency: String,
    #[serde(with = "time::serde::rfc3339")]
    pub captured_at: OffsetDateTime,
}

/// A product as listed in a particular store's catalogue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreProduct {
    pub product_id: ProductId,
    pub store_id: StoreId,
    pub store_sku: String,
    pub url: String,
    #[serde(with = "time::serde::rfc3339")]
    pub last_scraped_at: OffsetDateTime,
}

/// Aggregated context used by scorers and evaluators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodContext {
    pub product_id: ProductId,
    pub nutrition: NutritionFacts,
    pub price: Option<PricePoint>,
    pub weight_g: f32,
}

/// Controls the level of browser impersonation used by a store adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpersonationTier {
    /// Plain HTTP client with standard headers.
    Http,
    /// Full headless-browser impersonation.
    Browser,
}

/// Raw product data as returned by a store adapter before normalisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawProduct {
    pub store_sku: String,
    pub ean: Option<String>,
    pub name: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub ingredients: Option<String>,
    pub nutrition: Option<NutritionFacts>,
    pub price: Option<f64>,
    pub unit_price: Option<f64>,
    pub currency: Option<String>,
    pub serving_size_g: Option<f32>,
    pub url: String,
}

/// A store category node as returned by a store adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCategory {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}
