use crate::{
    config::ConfigResponse,
    state::{AppState, Place},
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

pub async fn poll(
    State(state): State<Arc<AppState>>,
    req: Request,
) -> Result<Json<ConfigResponse>, StatusCode> {
    let headers_map = req.headers();
    let place_guid = headers_map
        .get("place-guid")
        .and_then(|hv| hv.to_str().ok())
        .unwrap();
    let place_name = headers_map
        .get("place-name")
        .and_then(|hv| hv.to_str().ok())
        .unwrap();
    let place_id_str = headers_map
        .get("place-id")
        .and_then(|hv| hv.to_str().ok())
        .unwrap();
    let place_id = place_id_str.parse::<u64>().unwrap();

    let place = Place {
        name: place_name.to_string(),
        id: place_id,
    };

    match state.active_place.lock().await.as_deref() {
        Some(active_place) if active_place == place_guid => Ok(Json(ConfigResponse {
            test_roots: state.config.roots.clone(),
            test_extra_options: state.config.test_extra_options.clone().unwrap_or_default(),
        })),
        _ => {
            state.places.insert(place_guid.to_string(), place);
            Err(StatusCode::FORBIDDEN)
        }
    }
}
