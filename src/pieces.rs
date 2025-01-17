#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChessPiece { Pawn = 1, Rook = 2, Knight = 3, Bishop = 4, Queen = 5, King = 6 }

impl ChessPiece {
    pub fn from(value: u8) -> Result<Option<Self>, &'static str> {
        match value {
            0 => Ok(None),
            1 => Ok(Some(Self::Pawn)),
            2 => Ok(Some(Self::Rook)),
            3 => Ok(Some(Self::Knight)),
            4 => Ok(Some(Self::Bishop)),
            5 => Ok(Some(Self::Queen)),
            6 => Ok(Some(Self::King)),
            _ => Err("not a valid piece")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChessPieceColor { Black, White }