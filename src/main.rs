use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use libchess::ChessBoard;
use serde::Serialize;
use tokio::sync::Mutex;

mod squares;
mod pieces;

//use squares::ToString;

use crate::pieces::SimplifiedMove;

type SharedBoard = Arc<Mutex<ChessBoard>>;

#[tokio::main]
async fn main() {
    let board = Arc::new(Mutex::new(ChessBoard::default()));

    let app = Router::new()
        .route("/moves", get(get_moves))
        .with_state(board);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to setup listener");
    axum::serve(listener, app).await.expect("Failed to serve app");
}

#[derive(Serialize)]
struct GetMovesResponse {
    success: bool,
    text: String,
    moves: Option<Vec<SimplifiedMove>>
}

async fn get_moves(
    State(board): State<SharedBoard>
) -> (StatusCode, Json<GetMovesResponse>) {
    let board = board.lock().await;

    let mut moves: Vec<SimplifiedMove> = Vec::new();
    for board_move in board.get_legal_moves() {
        let piece_move = match board_move.piece_move() {
            Ok(mov) => mov,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(GetMovesResponse {
                success: false,
                text: format!("Failed to get possible moves: {}", e),
                moves: None
            }))
        };

        let source = piece_move.get_source_square().to_string().to_uppercase();
        let destination = piece_move.get_destination_square().to_string().to_uppercase();

        moves.push(SimplifiedMove { piece_type: piece_move.get_piece_type(), source, destination });
    }

    (StatusCode::OK, Json(GetMovesResponse {
        success: true,
        text: "Successfully fetched possible moves".to_string(),
        moves: Some(moves)
    }))
}
