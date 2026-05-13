use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::{SharedApiState, pieces::SimplifiedMove};

#[derive(Serialize)]
pub struct GetMovesResponse {
    success: bool,
    text: String,
    moves: Option<Vec<SimplifiedMove>>,
    side_to_move: String
}

pub async fn get_legal_moves(
    State(state): State<SharedApiState>
) -> (StatusCode, Json<GetMovesResponse>) {
    let board = state.lock().await.board;

    let side_to_move = board.get_side_to_move().to_string();

    let mut moves: Vec<SimplifiedMove> = Vec::new();
    for board_move in board.get_legal_moves() {
        let piece_move = match board_move.piece_move() {
            Ok(mov) => mov,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(GetMovesResponse {
                success: false,
                text: format!("Failed to get possible moves: {}", e),
                moves: None,
                side_to_move
            }))
        };

        let source = piece_move.get_source_square().to_string().to_uppercase();
        let destination = piece_move.get_destination_square().to_string().to_uppercase();

        moves.push(SimplifiedMove { piece_type: piece_move.get_piece_type(), source, destination });
    }

    (StatusCode::OK, Json(GetMovesResponse {
        success: true,
        text: "Successfully fetched possible moves".to_string(),
        moves: Some(moves),
        side_to_move
    }))
}

pub async fn get_last_moves(
    State(state): State<SharedApiState>
) -> (StatusCode, Json<GetMovesResponse>) {
    let state = state.lock().await;

    let side_to_move = state.board.get_side_to_move().to_string();

    (
        StatusCode::OK,
        Json(GetMovesResponse { success: true, text: "Successfully fetched past moves".to_string(), moves: Some(state.moves.clone()), side_to_move })
    )
}
