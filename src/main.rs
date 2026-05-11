use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use libchess::ChessBoard;
use tokio::sync::Mutex;

mod squares;
mod pieces;
mod endpoints;

use crate::endpoints::moves::get_legal_moves;

type SharedBoard = Arc<Mutex<ChessBoard>>;

#[tokio::main]
async fn main() {
    let board = Arc::new(Mutex::new(ChessBoard::default()));

    let app = Router::new()
        .route("/moves/get_legal", get(get_legal_moves))
        .with_state(board);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to setup listener");
    axum::serve(listener, app).await.expect("Failed to serve app");
}
