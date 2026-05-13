use std::str::FromStr;

use libchess::{BoardMove, PieceMove, PieceType, Square, errors::LibChessError, mv};
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

#[derive(Serialize, Deserialize, Clone)]
pub struct SimplifiedMove {
    #[serde(with = "PieceTypeDef")]
    pub piece_type: PieceType,
    pub source: String,
    pub destination: String,
}

pub trait FromSimplifiedMove {
    fn from_simplified(value: SimplifiedMove) -> Result<BoardMove, LibChessError>;
}

impl FromSimplifiedMove for BoardMove {
    fn from_simplified(value: SimplifiedMove) -> Result<BoardMove, LibChessError> {
        let source = Square::from_str(value.source.as_str())?;
        let destination = Square::from_str(value.destination.as_str())?;

        Ok(mv!(value.piece_type, source, destination))
    }
}
