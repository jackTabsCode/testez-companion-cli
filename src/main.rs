use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use console::style;
use dashmap::DashMap;
use inquire::Select;
use state::AppState;
use std::{net::SocketAddr, process::exit, sync::Arc, time::Duration};
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

        eprintln!("{}", style("Waiting for place(s) to check in...").dim());

        loop {
            if start_time.elapsed() < Duration::from_secs(1) {
                continue;
            }

            if start_time.elapsed() > Duration::from_secs(5) {
                eprintln!(
                    "{}",
                    style("No places have reported anything. Studio might not be open?").red()
                );
                exit(1);
            }

            let key: Option<String> = match state_clone.places.len() {
                0 => None,
                1 => Some(state_clone.places.iter().next().unwrap().key().to_string()),
                _ => Some(inquire_place(Arc::clone(&state_clone))),
            };

            match key {
                Some(key) => {
                    eprintln!(
                        "{}",
                        style(format!("Waiting for results from place {}...", key)).dim(),
                    );
                    state_clone.active_place.lock().await.replace(key);
                    break;
                }
                None => {
                    continue;
                }
            }
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
        .unwrap();
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
