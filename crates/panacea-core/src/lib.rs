pub mod errors;
pub mod events;
pub mod traits;
pub mod types;

pub use errors::{AdapterError, PanaceaError};
pub use events::{
    EventEnvelope, MealLoggedPayload, NutritionNormalisedPayload, PriceChangedPayload,
    ProductScrapedPayload, ProductScoredPayload,
};
pub use traits::{Scorer, StoreAdapter};
pub use types::{
    EanCode, FoodContext, ImpersonationTier, MicronutrientFact, NutritionFacts, PricePoint,
    ProductId, RawCategory, RawProduct, StoreId, StoreProduct,
};
