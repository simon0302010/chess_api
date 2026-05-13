use axum::{Json, extract::State, http::StatusCode};
use libchess::BoardMove;
use serde::Serialize;

use crate::{
    SharedApiState,
    pieces::{FromSimplifiedMove, SimplifiedMove, FromBoardMove},
};

#[derive(Serialize)]
pub struct GetMovesResponse {
    success: bool,
    text: String,
    moves: Option<Vec<SimplifiedMove>>,
    side_to_move: String,
}

pub async fn get_legal_moves(
    State(state): State<SharedApiState>,
) -> (StatusCode, Json<GetMovesResponse>) {
    let game = state.lock().await.game.clone();

    let side_to_move = game.get_side_to_move().to_string();

    let mut moves: Vec<SimplifiedMove> = Vec::new();
    for board_move in game.get_legal_moves() {
        let piece_move = match board_move.piece_move() {
            Ok(mov) => mov,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GetMovesResponse {
                        success: false,
                        text: format!("Failed to get possible moves: {}", e),
                        moves: None,
                        side_to_move,
                    }),
                );
            }
        };

        let source = piece_move.get_source_square().to_string().to_uppercase();
        let destination = piece_move
            .get_destination_square()
            .to_string()
            .to_uppercase();

        moves.push(SimplifiedMove {
            piece_type: piece_move.get_piece_type(),
            source,
            destination,
        });
    }

    (
        StatusCode::OK,
        Json(GetMovesResponse {
            success: true,
            text: "Successfully fetched possible moves".to_string(),
            moves: Some(moves),
            side_to_move,
        }),
    )
}

pub async fn get_last_moves(
    State(state): State<SharedApiState>,
) -> (StatusCode, Json<GetMovesResponse>) {
    let state = state.lock().await;

    let side_to_move = state.game.get_side_to_move().to_string();

    let moves = state.game
        .get_action_history()
        .get_moves()
        .iter()
        .filter_map(|m| SimplifiedMove::from_board_move(m.clone()).ok())
        .collect();

    (
        StatusCode::OK,
        Json(GetMovesResponse {
            success: true,
            text: "Successfully fetched past moves".to_string(),
            moves: Some(moves),
            side_to_move,
        }),
    )
}

#[derive(Serialize)]
pub struct MakeMoveResponse {
    success: bool,
    text: String,
    next_side: String,
}

pub async fn make_move(
    State(state): State<SharedApiState>,
    Json(payload): Json<SimplifiedMove>,
) -> (StatusCode, Json<MakeMoveResponse>) {
    let mut state = state.lock().await;

    let board_move = match BoardMove::from_simplified(payload.clone()) {
        Ok(mov) => mov,
        Err(e) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(MakeMoveResponse {
                    success: false,
                    text: format!("Failed to parse source or destination: {}", e),
                    next_side: state.game.get_side_to_move().to_string(),
                }),
            );
        }
    };

    match state.game.make_move(&libchess::Action::MakeMove(board_move)) {
        Ok(_) => {
            let piece_move = match board_move.piece_move() {
                Ok(mov) => mov,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(MakeMoveResponse {
                            success: false,
                            text: format!("Failed to move {}: {}", payload.piece_type, e),
                            next_side: state.game.get_side_to_move().to_string(),
                        }),
                    );
                }
            };

            (
                StatusCode::OK,
                Json(MakeMoveResponse {
                    success: true,
                    text: format!(
                        "Successfully moved {:?} from {} to {}",
                        payload.piece_type,
                        piece_move.get_source_square().to_string().to_uppercase(),
                        piece_move
                            .get_destination_square()
                            .to_string()
                            .to_uppercase()
                    ),
                    next_side: state.game.get_side_to_move().to_string(),
                }),
            )
        }
        Err(e) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(MakeMoveResponse {
                success: false,
                text: format!("{}", e),
                next_side: state.game.get_side_to_move().to_string(),
            }),
        ),
    }
}
