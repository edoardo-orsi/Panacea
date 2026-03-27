use crate::errors::AdapterError;
use crate::types::{FoodContext, ImpersonationTier, RawCategory, RawProduct};

/// Implemented by each store-specific scraping adapter.
///
/// All methods are async. The `#[allow(async_fn_in_trait)]` attribute is used
/// instead of `async_trait` because the project targets Rust 1.75+ where
/// return-position `impl Trait` in traits is stabilised.
#[allow(async_fn_in_trait)]
pub trait StoreAdapter: Send + Sync {
    /// A unique, stable identifier for the store (e.g. `"lidl_de"`).
    fn store_id(&self) -> &'static str;

    /// The impersonation strategy required to scrape this store without
    /// triggering bot-detection measures.
    fn impersonation_tier(&self) -> ImpersonationTier;

    /// Search the store catalogue for products matching `query`.
    async fn search_products(&self, query: &str) -> Result<Vec<RawProduct>, AdapterError>;

    /// Fetch a single product by its store-internal SKU.
    async fn fetch_product(&self, store_sku: &str) -> Result<RawProduct, AdapterError>;

    /// Fetch the full category tree for the store.
    async fn fetch_categories(&self) -> Result<Vec<RawCategory>, AdapterError>;
}

/// Implemented by each KPI-scoring algorithm.
pub trait Scorer: Send + Sync {
    /// The serialisable score output produced by this scorer.
    type Output: serde::Serialize;

    /// A unique, stable name for this scorer (e.g. `"nutriscore"`).
    fn name(&self) -> &'static str;

    /// Monotonically increasing version number; bump when scoring logic
    /// changes in a way that invalidates previously stored scores.
    fn version(&self) -> u32;

    /// Compute a score for the given food context.
    fn score(&self, food: &FoodContext) -> Self::Output;
}
