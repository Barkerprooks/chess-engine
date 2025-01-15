use crate::pieces::{ChessPiece};
use crate::board::{ChessBoard, ChessTile};
use crate::math::{search_grid_plus, search_grid_diag, V2};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct ChessMove (pub u16);

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ChessMoveExt {
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Captures,
    EpCapture
}

#[allow(dead_code)]
impl ChessMove {

    fn pawn_moves(src: &V2, board: ChessBoard) -> Vec<V2> {
        // generate valid pawn moves then add and subtract valid cells depending on
        // the weird pawn rules

        let tile: ChessTile = board.tile(src);
        
        let direction: i8 = match Some(board.player_color()) == tile.color() {
            true => -1,
            false => 1,
        };

        let one_step = i8::try_from(src.y).unwrap() + direction;

        let mut moves = match tile.has_moved() {
            true => vec![V2 { x: src.x, y: u16::try_from(one_step).unwrap() }],
            false => {
                let two_step = i8::try_from(src.y).unwrap() + direction * 2;
                vec![
                    V2 { x: src.x, y: u16::try_from(one_step).unwrap() },
                    V2 { x: src.x, y: u16::try_from(two_step).unwrap() } 
                ]
            }
        };

        // exclude spots where there are pieces already
        // get the furthest spot first and remove it 
        for _ in 0..2 {
            if board.tile(moves.last().unwrap()).color() != None {
                moves.pop();
            }

            if moves.len() == 0 {
                break;
            }
        }

        let horizontal = [
            i8::try_from(src.x).unwrap() - 1, 
            i8::try_from(src.x).unwrap() + 1
        ];

        // find diagonals where the peice can attack
        for x in horizontal {
            if x >= 0 && x < 8 {
                let v2 = V2 { 
                    x: u16::try_from(x).unwrap(),
                    y: u16::try_from(one_step).unwrap()
                };

                let color = board.tile(&v2).color();

                if color != None && tile.color() != color {
                    moves.push(v2);
                }
            }
        }

        moves
    }

    fn knight_moves(src: &V2, board: ChessBoard) -> Vec<V2> {
        
        let tile_color = board.tile(src).color();

        let moves = vec![
            V2::get_offset(src, -1,  2), V2::get_offset(src, 1,  2),
            V2::get_offset(src, -2,  1), V2::get_offset(src, 2,  1),
            V2::get_offset(src, -1, -2), V2::get_offset(src, 1, -2),
            V2::get_offset(src, -2, -1), V2::get_offset(src, 2, -1)
        ];

        moves.iter()
            .filter(|valid_move| {
                match valid_move {
                    Some(valid_move) => board.tile(valid_move).color() != tile_color,
                    None => false
                }
            })
            .map(|valid_move| valid_move.unwrap())
            .collect()
    }

    fn queen_moves(src: &V2, board: ChessBoard) -> Vec<V2> {
        // just merge the plus and diagonal search
        search_grid_plus(src, board).into_iter()
            .chain(search_grid_diag(src, board).into_iter())
            .collect()
    }

    fn king_moves(src: &V2, board: ChessBoard) -> Vec<V2> {
        
        for y in src.y - 1..src.y + 1 {
            for x in src.x - 1..src.x + 1 {
                let dst = V2::from_i8(x as i8, y as i8);
                if dst != None && dst != Some(*src) {
                    let d = dst.unwrap();
                    println!("({}, {})", d.x, d.y);
                }
            }
        }

        vec![]
    }

    fn illegal_move(src: &V2, dst: &V2, board: ChessBoard) -> bool {
        // check for each type of piece
        let valid_moves = match board.tile(src).piece() {
            Some(ChessPiece::Pawn) => Self::pawn_moves(src, board),
            Some(ChessPiece::Rook) => search_grid_plus(src, board),
            Some(ChessPiece::Knight) => Self::knight_moves(src, board),
            Some(ChessPiece::Bishop) => search_grid_diag(src, board),
            Some(ChessPiece::Queen) => Self::queen_moves(src, board),
            Some(ChessPiece::King) => vec![],
            None => vec![] // cant move an empty space
        };

        // if the coordinate is included, it's valid.
        // invert the valid signal to return the correct
        // value
        !valid_moves.iter().any(|valid_move| valid_move == dst)
    }

    fn get_ext(src: &V2, dst: &V2, board: ChessBoard) -> ChessMoveExt {
        let tiles = board.tile_pair(src, dst);

        // check to see if the pawn has been moved two spots in any y direction
        // dont need to be exact, if the move is invalid it'll be discarded in the
        // larger context
        let src_y = i16::try_from(src.y).unwrap();
        let dst_y = i16::try_from(dst.y).unwrap();

        // if a pawn moves twice
        if tiles.0.piece() == Some(ChessPiece::Pawn) && (src_y - dst_y).abs() == 2 {
            return ChessMoveExt::DoublePawnPush
        }

        // if a peice gets capped
        if tiles.1.piece() != None && tiles.0.color() != tiles.1.color() {
            return ChessMoveExt::Captures
        }

        ChessMoveExt::Quiet
    }

    pub fn new(src: &V2, dst: &V2, board: ChessBoard) -> Result<Self, &'static str> {
        if Self::illegal_move(src, dst, board) {
            return Err("illegal move")
        }
        
        let ext = Self::get_ext(src, dst, board) as u16;

        Ok(Self::raw(src, dst, ext))
    }

    pub fn raw(src: &V2, dst: &V2, ext: u16) -> Self {
        Self(src.x | (src.y << 3) | (dst.x << 6) | (dst.y << 9) | (ext << 12))
    }

    pub fn ext(&self) -> ChessMoveExt { 
        ChessMoveExt::from((self.0 >> 12) & 0x15)
    }

    pub fn src(&self) -> V2 {
        V2 { x: self.0 & 0x7, y: self.0 >> 3 & 0x7 }
    }

    pub fn dst(&self) -> V2 {
        V2 { x: self.0 >> 6 & 0x7, y: self.0 >> 9 & 0x7 }
    }
}

#[allow(dead_code)]
impl ChessMoveExt {
    pub fn from(value: u16) -> ChessMoveExt {
        match value {
            1 => ChessMoveExt::DoublePawnPush,
            2 => ChessMoveExt::KingCastle,
            3 => ChessMoveExt::QueenCastle,
            4 => ChessMoveExt::Captures,
            5 => ChessMoveExt::EpCapture,
            _ => ChessMoveExt::Quiet
        }
    }
}