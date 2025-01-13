use crate::pieces::{ChessPiece, ChessPieceColor};
use crate::moves::{ChessMove, ChessMoveOption};
use crate::math::V2;

const INITIAL_LAYOUT: [u8; 64] = [
    2, 3, 4, 5, 6, 4, 3, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    2, 3, 4, 5, 6, 4, 3, 2
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChessTile (u8);

pub struct ChessBoard {
    turn: u8, // keeping track of the current turn
    tiles: [ChessTile; 64], // 8x8 board
    moves: [ChessMove; 50], // game ends at 50 moves
}

impl ChessTile {
    pub fn new(piece: Option<ChessPiece>, color: Option<ChessPieceColor>) -> Result<Self, &'static str> {
        match piece {
            Some(piece) => match color {
                Some(ChessPieceColor::Black) => Ok(ChessTile(piece as u8 | 1 << 3)),
                Some(ChessPieceColor::White) => Ok(ChessTile(piece as u8 | 2 << 3)),
                None => Err("occupied tile requires a color")
            },
            None => Ok(ChessTile(0))
        }
    }

    pub fn piece(&self) -> Option<ChessPiece> {
        ChessPiece::from(self.0 & 0x7)
            .expect("not a valid chess piece")
    }

    pub fn color(&self) -> Option<ChessPieceColor> {
        match self.0 >> 3 {
            1 => Some(ChessPieceColor::Black),
            2 => Some(ChessPieceColor::White),
            _ => None
        }
    }
}

fn convert_to_tile(index: usize, value: &u8, colors: (ChessPieceColor, ChessPieceColor)) -> ChessTile {
    let color = match index {
        0..16 => Some(colors.1),
        48..64 => Some(colors.0),
        _ => None
    };

    let piece = ChessPiece::from(*value)
        .expect("not a valid piece");

    ChessTile::new(piece, color)
        .expect("error creating tile")
}

impl ChessBoard {
    pub fn new(player_color: ChessPieceColor) -> Self {

        let opponent_color = match player_color {
            ChessPieceColor::White => ChessPieceColor::Black,
            ChessPieceColor::Black => ChessPieceColor::White
        };

        let colors = (player_color, opponent_color);

        ChessBoard {
            tiles: INITIAL_LAYOUT.iter()
                .enumerate()
                .map(|(index, value)| convert_to_tile(index, value, colors))
                .collect::<Vec<ChessTile>>()
                .try_into()
                .unwrap(),
            moves: [ChessMove(0); 50],
            turn: 0
        }
    }

    pub fn clear(&mut self, src: &V2) {
        let x = (*src).x as usize; 
        let y = (*src).y as usize;
        self.tiles[x+ 8 * y] = ChessTile(0)
    }

    pub fn place(&mut self, src: &V2, tile: ChessTile) {
        let x = (*src).x as usize; 
        let y = (*src).y as usize;
        self.tiles[x + 8 * y] = tile;
    }

    pub fn get_tile(&self, src: &V2) -> ChessTile {
        let x = (*src).x as usize; 
        let y = (*src).y as usize;
        self.tiles[x+ 8 * y]
    }

    pub fn put_tile(&mut self, src: &V2, dst: &V2) -> bool {

        // TODO: make sure the move is legal

        // TODO: check to see if a piece is captured

        self.place(dst, self.get_tile(src));
        self.clear(src);

        self.moves[self.turn as usize] = ChessMove::new(src, dst, ChessMoveOption::Quiet);
        self.turn += 1;

        true
    }

    pub fn show(&self) {
        println!();
        for y in 0..8 {
            for x in 0..8 {
                match self.tiles[x + 8 * y].piece() {
                    Some(piece) => match piece {
                        ChessPiece::Pawn => print!("Pn"),
                        ChessPiece::Rook => print!("Rk"),
                        ChessPiece::Knight => print!("Kt"),
                        ChessPiece::Bishop => print!("Bp"),
                        ChessPiece::Queen => print!("Qn"),
                        ChessPiece::King => print!("Kg"),
                    },
                    None => print!("__")
                };
            }
            println!();
        }
        println!();
    }
}