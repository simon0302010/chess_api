use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use libchess::{ChessBoard, Game};
use tokio::sync::Mutex;

mod endpoints;
mod pieces;
mod squares;

use crate::endpoints::moves::*;

type SharedApiState = Arc<Mutex<ApiState>>;

#[derive(Default)]
struct ApiState {
    game: Game,
    //moves: Vec<crate::pieces::SimplifiedMove>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ApiState::default()));

    let app = Router::new()
        .route("/moves/available", get(get_legal_moves))
        .route("/moves/history", get(get_last_moves))
        .route("/move", post(make_move))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to setup listener");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve app");
}
