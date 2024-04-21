use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use dashmap::DashMap;
use inquire::Select;
use state::AppState;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{fs::read_to_string, spawn, sync::Mutex, time::Instant};

mod api;
mod config;
mod state;
mod testez;

#[tokio::main]
async fn main() {
    let config: Arc<Config> = {
        let contents = read_to_string("testez-companion.toml")
            .await
            .expect("Missing testez-companion.toml");

        Arc::new(toml::from_str(&contents).expect("Failed to parse testez-companion.toml"))
    };

    let state = Arc::new(AppState {
        config,
        places: DashMap::new(),
        active_place: Mutex::new(None),
    });

    let state_clone = Arc::clone(&state);
    spawn(async move {
        let start_time = Instant::now();

        loop {
            if start_time.elapsed() < Duration::from_secs(1) {
                continue;
            }

            let key = inquire_place(Arc::clone(&state_clone));
            state_clone.active_place.lock().await.replace(key);

            break;
        }
    });

    let app = Router::new()
        .route("/poll", get(api::poll))
        .route("/logs", post(api::logs))
        .route("/results", post(api::results))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 28859));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect(format!("Failed to bind to {}... is the port already in use?", addr).as_str());
}

fn inquire_place(state: Arc<AppState>) -> String {
    let options: Vec<String> = state
        .places
        .iter()
        .map(|place| {
            format!(
                "{} ({}) [{}]",
                place.value().name,
                place.value().id,
                place.key()
            )
        })
        .collect();

    let selected = Select::new("Select a place to run tests on:", options)
        .prompt()
        .expect("Failed to prompt user for place selection");

    let key = selected
        .split_whitespace()
        .last()
        .unwrap()
        .trim_matches(|c| c == '[' || c == ']');

    key.to_string()
}
