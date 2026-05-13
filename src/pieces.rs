use libchess::PieceType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "PieceType")]
enum PieceTypeDef {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Serialize, Clone)]
pub struct SimplifiedMove {
    #[serde(with = "PieceTypeDef")]
    pub piece_type: PieceType,
    pub source: String,
    pub destination: String
}
