mod middleware;
mod state;
mod translate;

use axum::{middleware::from_extractor_with_state, routing::post, Router};
use middleware::RequireAuth;
pub use state::AppState;

pub fn router<T>(state: AppState) -> Router {
    Router::new()
        .route("/translate", post(translate::translate_free))
        .route("/v1/translate", post(translate::translate_pro))
        .route("/v2/translate", post(translate::translate_official))
        .route_layer(from_extractor_with_state::<RequireAuth, AppState>(
            state.clone(),
        ))
        .with_state(state)
}
