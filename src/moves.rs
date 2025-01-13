use crate::math::V2;

#[derive(Copy, Clone)]
pub struct ChessMove (pub u16);

#[derive(Debug, PartialEq)]
pub enum ChessMoveOption {
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Captures,
    EpCapture
}

impl ChessMoveOption {
    pub fn from(value: u16) -> ChessMoveOption {
        match value {
            1 => ChessMoveOption::DoublePawnPush,
            2 => ChessMoveOption::KingCastle,
            3 => ChessMoveOption::QueenCastle,
            4 => ChessMoveOption::Captures,
            5 => ChessMoveOption::EpCapture,
            _ => ChessMoveOption::Quiet,
        }
    }
}

impl ChessMove {
    pub fn new(src: &V2, dst: &V2, option: ChessMoveOption) -> Self {
        ChessMove(src.x      | 
                  src.y << 3 | 
                  dst.x << 6 | 
                  dst.y << 9 |
                  (option as u16) << 12)
    }

    pub fn option(&self) -> ChessMoveOption { 
        ChessMoveOption::from(self.0 >> 12 & 0x15)
    }

    pub fn src(&self) -> V2 {
        V2 { x: self.0 & 0x7, y: self.0 >> 3 & 0x7 }
    }

    pub fn dst(&self) -> V2 {
        V2 { x: self.0 >> 6 & 0x7, y: self.0 >> 9 & 0x7 }
    }
}